use std::sync::Arc;
use std::sync::LazyLock;

use alistral_core::models::client::AlistralClient;
use futures::executor::block_on;
use interzic::InterzicClient;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use musicbrainz_db_lite::DBClient;

use crate::api::youtube::INTERZIC_DB;
use crate::api::youtube::TOKENCACHE;
use crate::api::youtube::YT_SECRET_FILE;
use crate::database::DB_LOCATION;
use crate::models::config::Config;

pub static ALISTRAL_CLIENT: LazyLock<AlistralClient> = LazyLock::new(|| block_on(create_client()));

pub async fn create_client() -> AlistralClient {
    let (listenbrainz, musicbrainz_rs, musicbrainz_db) = {
        let config = Config::load_or_panic();
        let config = config.read_or_panic();

        let listenbrainz = Arc::new(ListenbrainzClient::new_with_url(&config.listenbrainz_url));

        let musicbrainz_rs: Arc<MusicBrainzClient> = Arc::new(Default::default());

        let musicbrainz_db = DBClient::builder()
            .as_file(DB_LOCATION.to_path_buf())
            .create_file()
            .expect("Couldn't create database file");

        (listenbrainz, musicbrainz_rs, musicbrainz_db)
    };

    let musicbrainz_db = Arc::new(
        musicbrainz_db
            .connect_and_migrate()
            .await
            .expect("Couldn't connect to database")
            .set_mb_client(musicbrainz_rs.as_ref().clone())
            .build(),
    );

    let interzic = Arc::new(
        create_interzic(
            musicbrainz_rs.clone(),
            listenbrainz.clone(),
            musicbrainz_db.clone(),
        )
        .await,
    );

    AlistralClient {
        musicbrainz_rs,
        listenbrainz,
        musicbrainz_db,
        interzic,
    }
}

async fn create_interzic(
    musicbrainz_rs: Arc<MusicBrainzClient>,
    listenbrainz: Arc<ListenbrainzClient>,
    musicbrainz_db: Arc<DBClient>,
) -> InterzicClient {
    let mut builder = InterzicClient::new_builder();
    builder
        .create_database_if_missing(&INTERZIC_DB)
        .expect("Couldn't create interzic database");
    builder
        .read_database(INTERZIC_DB.clone().to_str().unwrap())
        .expect("Couldn't connect to interzic database");
    builder
        .migrate_database()
        .await
        .expect("Couldn't migrate interzic database");

    let mut client = builder.build().expect("Couldn't initialize Interzic");

    client.set_musicbrainz_client(musicbrainz_rs);
    client.set_listenbrainz_client(listenbrainz);
    client.set_musicbrainz_db_lite_client(musicbrainz_db);
    client
        .set_youtube_client(&YT_SECRET_FILE, &TOKENCACHE)
        .await
        .expect("Couldn't create the youtube client");

    client
}
