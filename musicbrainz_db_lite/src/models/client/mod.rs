use core::str::FromStr as _;
use core::time::Duration;
use std::path::PathBuf;
use std::sync::Arc;

use listenbrainz_rs::client::ListenBrainzClient;
use musicbrainz_rs::MusicBrainzClient;
use sequelles::databases::sqlite::database::ArcSqliteDatabase;
use sequelles::databases::sqlite::database::GetConnectionError;
use sequelles::databases::sqlite::database::SqliteDatabase;
use sequelles::databases::sqlite::pool::SqlitePoolConnection;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;

#[derive(Debug)]
pub struct DBClient {
    pub database: ArcSqliteDatabase,

    pub musicbrainz_client: Arc<MusicBrainzClient>,
    pub listenbrainz_client: Arc<ListenBrainzClient>,
}

impl DBClient {
    /// Open a database using its path
    pub fn from_path(
        path: PathBuf,
        musicbrainz_client: Arc<MusicBrainzClient>,
        listenbrainz_client: Arc<ListenBrainzClient>,
    ) -> Result<Self, sqlx::Error> {
        let optconn = SqliteConnectOptions::from_str(path.to_string_lossy().as_ref())?
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true)
            .foreign_keys(true)
            .busy_timeout(Duration::from_millis(60000));

        let db = SqliteDatabase::builder()
            .connection_config(optconn)
            .path(path)
            .migrations(sqlx::migrate!("./migrations"))
            .build();

        Ok(Self {
            database: Arc::new(db),
            listenbrainz_client,
            musicbrainz_client,
        })
    }

    /// Open a database in memory
    pub fn in_memory(
        name: &str,
        musicbrainz_client: Arc<MusicBrainzClient>,
        listenbrainz_client: Arc<ListenBrainzClient>,
    ) -> Result<Self, sqlx::Error> {
        let optconn =
            SqliteConnectOptions::from_str(&format!("file:{name}?mode=memory&cache=shared"))?
                .journal_mode(SqliteJournalMode::Wal)
                .foreign_keys(true)
                .busy_timeout(Duration::from_millis(60000));

        let db = SqliteDatabase::builder()
            .connection_config(optconn)
            .migrations(sqlx::migrate!("./migrations"))
            .build();

        Ok(Self {
            database: Arc::new(db),
            listenbrainz_client,
            musicbrainz_client,
        })
    }

    /// Get a connection to the database
    pub async fn get_conn(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        self.database.get_conn().await
    }

    /// Get a connection to the database
    pub async fn get_conn_as_task(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        let db = self.database.clone();
        tokio::spawn(async move { db.get_conn().await })
            .await
            .unwrap()
    }

    pub async fn get_raw_connection(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        self.database.get_conn().await
    }

    /// Default client with default settings. This create an in memory database, and is useful for testing
    pub fn get_default() -> Result<DBClient, sqlx::Error> {
        DBClient::in_memory(
            "",
            Arc::new(MusicBrainzClient::default()),
            Arc::new(ListenBrainzClient::default()),
        )
    }

    /// Default client with default settings. This create an in memory database, and is useful for testing.
    ///
    /// It separate the database from others by using a name, useful to run test in parallel
    pub fn get_default_named(name: &str) -> Result<DBClient, sqlx::Error> {
        DBClient::in_memory(
            name,
            Arc::new(MusicBrainzClient::default()),
            Arc::new(ListenBrainzClient::default()),
        )
    }
}
