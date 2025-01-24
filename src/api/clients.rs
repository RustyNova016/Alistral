use std::sync::LazyLock;

use alistral_core::models::client::AlistralClient;
use futures::executor::block_on;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use musicbrainz_db_lite::DBClient;

use crate::database::DB_LOCATION;
use crate::models::config::Config;

pub static ALISTRAL_CLIENT: LazyLock<AlistralClient> = LazyLock::new(|| block_on(create_client()));

pub async fn create_client() -> AlistralClient {
    let (listenbrainz, musicbrainz_rs, musicbrainz_db) = {
        let config = Config::load_or_panic();
        let config = config.read_or_panic();

        let listenbrainz = ListenbrainzClient::new_with_url(&config.listenbrainz_url);

        let musicbrainz_rs: MusicBrainzClient = Default::default();

        let musicbrainz_db = DBClient::builder()
            .as_file(DB_LOCATION.to_path_buf())
            .create_file()
            .expect("Couldn't create database file");

        (listenbrainz, musicbrainz_rs, musicbrainz_db)
    };

    let musicbrainz_db = musicbrainz_db
        .connect_and_migrate()
        .await
        .expect("Couldn't connect to database")
        .set_mb_client(musicbrainz_rs.clone())
        .build();

    AlistralClient {
        musicbrainz_rs,
        listenbrainz,
        musicbrainz_db,
    }
}
