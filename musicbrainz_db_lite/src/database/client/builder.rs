use core::marker::PhantomData;
use core::str::FromStr as _;
use core::time::Duration;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_rs_nova::client::MusicBrainzClient;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::Connection as _;
use sqlx::SqliteConnection;

use crate::database::db_connection::DbConnection;

use super::DBClient;

#[derive(Debug, Default)]
pub struct ClientBuilder<Loc = (), DbConn = (), MBClient = (), LBClient = ()> {
    pub connection: DbConn,

    pub musicbrainz_client: MBClient,

    pub listenbrainz_client: LBClient,

    pub database_type: PhantomData<Loc>,

    pub database_location: Option<PathBuf>,
}

// Type states types

// --- Location
pub struct DatabaseFile;
pub struct InMemory;

impl<DbConn, MBClient, LBClient> ClientBuilder<(), DbConn, MBClient, LBClient> {
    pub fn in_memory(self) -> ClientBuilder<InMemory, DbConn, MBClient, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_type: Default::default(),
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
            database_location: None,
        }
    }

    pub fn as_file(self, path: PathBuf) -> ClientBuilder<DatabaseFile, DbConn, MBClient, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_type: Default::default(),
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
            database_location: Some(path),
        }
    }
}

impl<MBClient, LBClient> ClientBuilder<DatabaseFile, (), MBClient, LBClient> {
    /// Create the database file if it doesn't exist
    pub fn create_file(self) -> Result<Self, crate::Error> {
        let loc = self.database_location.clone().unwrap();

        if !loc.try_exists()? {
            File::create_new(loc)?;
        }

        Ok(self)
    }

    pub async fn connect(
        self,
    ) -> Result<ClientBuilder<DatabaseFile, DbConnection, MBClient, LBClient>, crate::Error> {
        let optconn = SqliteConnectOptions::from_str(
            self.database_location.clone().unwrap().to_str().unwrap(),
        )?
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_millis(60000));

        let connection = SqliteConnection::connect_with(&optconn).await?;

        Ok(ClientBuilder {
            connection: DbConnection::new(connection),
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        })
    }

    pub async fn connect_and_migrate(
        self,
    ) -> Result<ClientBuilder<DatabaseFile, DbConnection, MBClient, LBClient>, crate::Error> {
        let new = self.connect().await?;

        musicbrainz_db_lite_schema::create_and_migrate(
            &mut *new.connection.acquire_guarded().await,
        )
        .await?;

        Ok(new)
    }
}

impl<MBClient, LBClient> ClientBuilder<InMemory, (), MBClient, LBClient> {
    pub async fn connect_and_migrate(
        self,
    ) -> Result<ClientBuilder<InMemory, DbConnection, MBClient, LBClient>, crate::Error> {
        let optconn = SqliteConnectOptions::from_str(":memory:")?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        let mut connection = SqliteConnection::connect_with(&optconn).await?;

        musicbrainz_db_lite_schema::create_and_migrate(&mut connection).await?;

        Ok(ClientBuilder {
            connection: DbConnection::new(connection),
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        })
    }
}

impl<Loc, DbConn, LBClient> ClientBuilder<Loc, DbConn, (), LBClient> {
    pub fn set_mb_client(
        self,
        musicbrainz_client: Arc<MusicBrainzClient>,
    ) -> ClientBuilder<Loc, DbConn, Arc<MusicBrainzClient>, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        }
    }

    pub fn set_default_mb_client(
        self,
    ) -> ClientBuilder<Loc, DbConn, Arc<MusicBrainzClient>, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: Arc::new(MusicBrainzClient::default()),
            listenbrainz_client: self.listenbrainz_client,
        }
    }
}

impl<Loc, DbConn, MBClient> ClientBuilder<Loc, DbConn, MBClient, ()> {
    pub fn set_lb_client(
        self,
        listenbrainz_client: Arc<ListenbrainzClient>,
    ) -> ClientBuilder<Loc, DbConn, MBClient, Arc<ListenbrainzClient>> {
        ClientBuilder {
            connection: self.connection,
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client,
        }
    }

    pub fn set_default_lb_client(
        self,
    ) -> ClientBuilder<Loc, DbConn, MBClient, Arc<ListenbrainzClient>> {
        ClientBuilder {
            connection: self.connection,
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: Arc::new(Default::default()),
        }
    }
}

impl<Loc> ClientBuilder<Loc, DbConnection, Arc<MusicBrainzClient>, Arc<ListenbrainzClient>> {
    pub fn build(self) -> DBClient {
        DBClient {
            connection: self.connection,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        }
    }
}
