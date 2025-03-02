use std::sync::Arc;
use std::sync::LazyLock;

use alistral_core::models::client::AlistralClient;
use futures::executor::block_on;
use interzic::InterzicClient;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use tracing::debug;

use crate::database::DB_LOCATION;
use crate::models::config::Config;
use crate::utils::constants::INTERZIC_DB;
use crate::utils::constants::TOKENCACHE;
use crate::utils::constants::YT_SECRET_FILE;
use crate::utils::env::in_offline_mode;

pub static ALISTRAL_CLIENT: LazyLock<AlistralClient> = LazyLock::new(|| block_on(create_client()));

pub async fn create_client() -> AlistralClient {
    debug!("Creating client");
    let (listenbrainz, musicbrainz_rs, musicbrainz_db) = {
        let config = Config::load_or_panic();
        let config = config.read_or_panic();

        let listenbrainz = Arc::new(ListenbrainzClient::new_with_url(&config.listenbrainz_url));

        let mut musicbrainz_rs = MusicBrainzClient::default();
        musicbrainz_rs.musicbrainz_url = config.musicbrainz_url.to_string();
        let musicbrainz_rs = Arc::new(musicbrainz_rs);

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
            .set_mb_client(musicbrainz_rs.clone())
            .set_lb_client(listenbrainz.clone())
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
    debug!("Created client");

    AlistralClient {
        musicbrainz_rs,
        listenbrainz,
        musicbrainz_db,
        interzic,

        offline: in_offline_mode(),
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

    if YT_SECRET_FILE.exists() {
        client
            .set_youtube_client(&YT_SECRET_FILE, &TOKENCACHE)
            .await
            .expect("Couldn't create the youtube client");
    }

    client
}
