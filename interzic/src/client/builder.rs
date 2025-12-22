use core::str::FromStr as _;
use core::time::Duration;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqliteJournalMode;
use sqlx::sqlite::SqlitePoolOptions;

use crate::InterzicClient;

#[derive(Default)]
pub struct ClientBuilder {
    pub database_client: Option<sqlx::SqlitePool>,
}

impl ClientBuilder {
    pub fn create_database_if_missing(&self, path: &Path) -> Result<(), io::Error> {
        if path.exists() {
            return Ok(());
        }

        match File::create_new(path) {
            Ok(_) => Ok(()),
            Err(err) => {
                if err.kind() == io::ErrorKind::AlreadyExists {
                    return Ok(());
                }

                Err(err)
            }
        }
    }

    pub fn read_database(&mut self, database_path: &str) -> Result<(), sqlx::Error> {
        let optconn = SqliteConnectOptions::from_str(database_path)?
            .journal_mode(SqliteJournalMode::Wal)
            .busy_timeout(Duration::from_millis(60000));

        let pool = SqlitePoolOptions::new()
            .acquire_timeout(Duration::from_millis(60000))
            .connect_lazy_with(optconn);
        self.database_client = Some(pool);

        Ok(())
    }

    pub async fn migrate_database(&self) -> Result<(), crate::Error> {
        Ok(sqlx::migrate!("./migrations")
            .run(self.database_client.as_ref().unwrap())
            .await?)
    }

    pub fn build(self) -> Result<InterzicClient, crate::Error> {
        Ok(InterzicClient {
            database_client: self
                .database_client
                .ok_or(crate::Error::ClientBuildingError(
                    "database_client".to_string(),
                ))?,
            musicbrainz_client: None,
            #[cfg(feature = "youtube")]
            youtube_client: None,
            listenbrainz_client: None,
            musicbrainz_db_lite_client: None,
            subsonic_clients: HashMap::new(),
        })
    }
}
