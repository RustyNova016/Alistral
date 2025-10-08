use std::path::PathBuf;
use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::MusicBrainzClient;
use musicbrainz_db_lite::SqlitePoolConnection;

use crate::database::DB_LOCATION;
use crate::models::client::AlistralCliClient;
use crate::utils::env::temp_database;

impl AlistralCliClient {
    pub(super) async fn create_mb_db_client(
        musicbrainz_client: Arc<MusicBrainzClient>,
        listenbrainz_client: Arc<ListenbrainzClient>,
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
}
