use core::str::FromStr as _;
use core::time::Duration;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_rs::client::MusicBrainzClient;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;

use crate::database::conn_pool::DBLitePool;
use crate::database::conn_pool::DBLitePoolManager;

use super::DBClient;

#[derive(Debug, Default)]
pub struct ClientBuilder<Loc = (), DbConn = (), MBClient = (), LBClient = ()> {
    pub connection: DbConn,

    pub musicbrainz_client: MBClient,

    pub listenbrainz_client: LBClient,

    pub database_type: Loc,

    pub database_location: Option<PathBuf>,
}

// Type states types

// --- Location
#[derive(Debug, Default)]
pub struct DatabaseFile;
#[derive(Debug, Default)]
pub struct InMemory(Option<String>);

impl<DbConn, MBClient, LBClient> ClientBuilder<(), DbConn, MBClient, LBClient> {
    /// Create a DB pool with all the connections being distinct in memory databases
    pub fn in_memory(self) -> ClientBuilder<InMemory, DbConn, MBClient, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_type: Default::default(),
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
            database_location: None,
        }
    }

    /// Create a DB pool with all the connections pointing to the same in memory database
    pub fn in_memory_with_name(
        self,
        name: String,
    ) -> ClientBuilder<InMemory, DbConn, MBClient, LBClient> {
        ClientBuilder {
            connection: self.connection,
            database_type: InMemory(Some(name)),
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
        pool_size: usize,
    ) -> Result<ClientBuilder<DatabaseFile, DBLitePool, MBClient, LBClient>, crate::Error> {
        let optconn = SqliteConnectOptions::from_str(
            self.database_location.clone().unwrap().to_str().unwrap(),
        )?
        .journal_mode(SqliteJournalMode::Wal)
        .busy_timeout(Duration::from_millis(60000));

        let connection = DBLitePoolManager::create_pool(optconn);
        connection.resize(pool_size);
        Ok(ClientBuilder {
            connection,
            database_location: self.database_location,
            database_type: self.database_type,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        })
    }

    pub async fn connect_and_migrate(
        self,
        pool_size: usize,
    ) -> Result<ClientBuilder<DatabaseFile, DBLitePool, MBClient, LBClient>, crate::Error> {
        let new = self.connect(pool_size).await?;

        musicbrainz_db_lite_schema::create_and_migrate(&mut *new.connection.get().await?).await?;

        Ok(new)
    }
}

impl<MBClient, LBClient> ClientBuilder<InMemory, (), MBClient, LBClient> {
    pub async fn connect_and_migrate(
        self,
        pool_size: usize,
    ) -> Result<ClientBuilder<InMemory, DBLitePool, MBClient, LBClient>, crate::Error> {
        let conn_str = match &self.database_type.0 {
            None => ":memory:".to_string(),
            Some(name) => format!("file:{name}?mode=memory&cache=shared"),
        };

        let optconn = SqliteConnectOptions::from_str(&conn_str)?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        let pool = DBLitePoolManager::create_pool(optconn);
        musicbrainz_db_lite_schema::create_and_migrate(&mut *pool.get().await?).await?;

        pool.resize(pool_size);
        Ok(ClientBuilder {
            connection: pool,
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

impl<Loc> ClientBuilder<Loc, DBLitePool, Arc<MusicBrainzClient>, Arc<ListenbrainzClient>> {
    pub fn build(self) -> DBClient {
        DBClient {
            database: self.connection,
            musicbrainz_client: self.musicbrainz_client,
            listenbrainz_client: self.listenbrainz_client,
        }
    }
}
