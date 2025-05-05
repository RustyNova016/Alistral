use std::path::PathBuf;
use std::sync::Arc;
use std::sync::LazyLock;

use alistral_core::AlistralClient;
use futures::executor::block_on;
use interzic::InterzicClient;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use symphonize::SymphonyzeClient;
use tuillez::fatal_error::IntoFatal;

use crate::database::DB_LOCATION;
use crate::models::config::Config;
use crate::models::config::config_trait::ConfigFile;
use crate::utils::constants::INTERZIC_DB;
use crate::utils::constants::TOKENCACHE;
use crate::utils::constants::YT_SECRET_FILE;
use crate::utils::env::in_offline_mode;
use crate::utils::env::temp_database;

pub static ALISTRAL_CLIENT: LazyLock<AlistralCliClient> =
    LazyLock::new(AlistralCliClient::create_blocking_or_fatal);

pub struct AlistralCliClient {
    pub config: Config,
    pub core: Arc<AlistralClient>,
    pub interzic: Arc<InterzicClient>,
    pub listenbrainz: Arc<ListenbrainzClient>,
    pub musicbrainz_db: Arc<DBClient>,
    pub symphonize: Arc<SymphonyzeClient>,
}

impl AlistralCliClient {
    pub async fn create() -> Result<Self, crate::Error> {
        let config = Config::load_unguarded()?;
        let musicbrainz = Self::create_mb_client(&config);
        let listenbrainz = Self::create_lb_client(&config);
        let musicbrainz_db =
            Self::create_mb_db_client(musicbrainz.clone(), listenbrainz.clone()).await;
        let interzic =
            Self::create_interzic(musicbrainz, listenbrainz.clone(), musicbrainz_db.clone()).await;
        let core = Self::create_core_client(musicbrainz_db.clone(), listenbrainz.clone());
        let symphonize = Self::create_symphonize_client(musicbrainz_db.clone());

        Ok(Self {
            config,
            core,
            interzic,
            listenbrainz,
            musicbrainz_db,
            symphonize,
        })
    }

    fn create_lb_client(config: &Config) -> Arc<ListenbrainzClient> {
        Arc::new(ListenbrainzClient::new_with_url(&config.listenbrainz_url))
    }

    fn create_mb_client(config: &Config) -> Arc<MusicBrainzClient> {
        let mut musicbrainz_rs = MusicBrainzClient::default();
        musicbrainz_rs.musicbrainz_url = config.musicbrainz_url.to_string();
        Arc::new(musicbrainz_rs)
    }

    async fn create_mb_db_client(
        musicbrainz: Arc<MusicBrainzClient>,
        listenbrainz: Arc<ListenbrainzClient>,
    ) -> Arc<DBClient> {
        //TODO: set db loaction in config
        let mut location = DB_LOCATION.to_path_buf();
        if temp_database() {
            location = PathBuf::from("./temp.db");
        }

        let musicbrainz_db = DBClient::builder()
            .as_file(location)
            .create_file()
            .expect("Couldn't create database file");

        let musicbrainz_db = musicbrainz_db
            .connect_and_migrate(64)
            .await
            .expect("Couldn't connect to database")
            .set_mb_client(musicbrainz)
            .set_lb_client(listenbrainz)
            .build();

        Arc::new(musicbrainz_db)
    }

    async fn create_interzic(
        musicbrainz_rs: Arc<MusicBrainzClient>,
        listenbrainz: Arc<ListenbrainzClient>,
        musicbrainz_db: Arc<DBClient>,
    ) -> Arc<InterzicClient> {
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

        Arc::new(client)
    }

    fn create_core_client(
        musicbrainz_db: Arc<DBClient>,
        listenbrainz: Arc<ListenbrainzClient>,
    ) -> Arc<AlistralClient> {
        Arc::new(AlistralClient {
            listenbrainz,
            musicbrainz_db,
            offline: in_offline_mode(),
        })
    }

    /// Create the client, or fancy panic if an error occur
    pub async fn create_or_fatal() -> Self {
        AlistralCliClient::create()
            .await
            .unwrap_fatal()
            .map_err(|err| err.panic())
            .unwrap()
    }

    /// Create the client as a blocking operation, or fancy panic if an error occur
    pub fn create_blocking_or_fatal() -> Self {
        block_on(Self::create_or_fatal())
    }

    pub fn create_symphonize_client(musicbrainz_db: Arc<DBClient>) -> Arc<SymphonyzeClient> {
        Arc::new(SymphonyzeClient {
            mb_database: musicbrainz_db,
        })
    }
}
