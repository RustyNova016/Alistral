use std::path::PathBuf;
use std::sync::Arc;

use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::ListenBrainzClient;
use musicbrainz_db_lite::MusicBrainzClient;
use musicbrainz_db_lite::SqlitePoolConnection;
use sqlx::Acquire;

use crate::database::DB_LOCATION;
use crate::models::client::AlistralCliClient;
use crate::utils::env::temp_database;

impl AlistralCliClient {
    pub(super) async fn create_mb_db_client(
        musicbrainz_client: Arc<MusicBrainzClient>,
        listenbrainz_client: Arc<ListenBrainzClient>,
    ) -> Arc<DBClient> {
        //TODO: set db location in config
        let mut location = DB_LOCATION.to_path_buf();
        if temp_database() {
            location = PathBuf::from("./temp.db");
        }

        let musicbrainz_db = DBClient::from_path(location, musicbrainz_client, listenbrainz_client)
            .expect("Couldn't create database client");

        Arc::new(musicbrainz_db)
    }

    /// Get a connection to the MB DB. Panics if it cannot get one
    pub async fn get_conn(&self) -> SqlitePoolConnection {
        self.musicbrainz_db.get_conn().await.unwrap()
    }

    pub async fn clean_up_mb_db(&self) {
        let mut conn = self.get_conn().await;
        let mut trans = conn.begin().await.unwrap();

        cleanup_table_data(&mut trans, "recordings").await.unwrap();
        cleanup_table_data(&mut trans, "artists").await.unwrap();
        cleanup_table_data(&mut trans, "releases").await.unwrap();
        cleanup_table_data(&mut trans, "labels").await.unwrap();

        trans.commit().await.unwrap();
    }
}

pub async fn cleanup_table_data(
    conn: &mut sqlx::SqliteConnection,
    table_name: &str,
) -> Result<(), sqlx::Error> {
    let sql = format!(
        "DELETE FROM {table_name} WHERE {table_name}.id IN (SELECT id FROM {table_name} WHERE full_update_date IS NOT NULL ORDER BY full_update_date LIMIT 10)"
    );
    sqlx::query(&sql).execute(&mut *conn).await?;

    let sql = format!(
        "DELETE FROM {table_name} WHERE {table_name}.id IN (SELECT id FROM {table_name} ORDER BY full_update_date LIMIT 10)"
    );
    sqlx::query(&sql).execute(conn).await?;

    Ok(())
}
