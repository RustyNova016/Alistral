use core::str::FromStr;
use std::sync::Arc;
use std::sync::LazyLock;

use alistral_core::AlistralClient;
use futures::executor::block_on;
#[cfg(feature = "interzic")]
use interzic::InterzicClient;
use musicbrainz_db_lite::DBClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use musicbrainz_db_lite::listenbrainz_rs::ListenBrainzClient;
#[cfg(feature = "musicbrainz")]
use symphonize::SymphonyzeClient;
use tuillez::fatal_error::IntoFatal;

use crate::models::config::Config;
use crate::models::config::config_trait::ConfigFile;

pub mod al_core;
#[cfg(feature = "interzic")]
pub mod interzic_client;
pub mod listenbrainz_rs;
pub mod mb_db;

pub static ALISTRAL_CLIENT: LazyLock<AlistralCliClient> =
    LazyLock::new(AlistralCliClient::create_blocking_or_fatal);

pub struct AlistralCliClient {
    pub config: Config,
    pub core: Arc<AlistralClient>,
    #[cfg(feature = "interzic")]
    pub interzic: Arc<InterzicClient>,
    pub listenbrainz: Arc<ListenBrainzClient>,
    pub musicbrainz_db: Arc<DBClient>,
    #[cfg(feature = "musicbrainz")]
    pub symphonize: Arc<SymphonyzeClient>,
}

impl AlistralCliClient {
    pub async fn create() -> Result<Self, crate::Error> {
        let config = Config::load_unguarded()?;
        let musicbrainz = Self::create_mb_client(&config);
        let listenbrainz = Self::create_lb_client(&config);
        let musicbrainz_db =
            Self::create_mb_db_client(musicbrainz.clone(), listenbrainz.clone()).await;
        #[cfg(feature = "interzic")]
        let interzic = Self::create_interzic(musicbrainz, musicbrainz_db.clone()).await;
        let core = Self::create_core_client(musicbrainz_db.clone());
        #[cfg(feature = "musicbrainz")]
        let symphonize = Self::create_symphonize_client(musicbrainz_db.clone());

        Ok(Self {
            config,
            core,
            #[cfg(feature = "interzic")]
            interzic,
            listenbrainz,
            musicbrainz_db,
            #[cfg(feature = "musicbrainz")]
            symphonize,
        })
    }

    fn create_mb_client(config: &Config) -> Arc<MusicBrainzClient> {
        let mut musicbrainz_rs = MusicBrainzClient::default();
        let url =
            url::Url::from_str(&config.musicbrainz_url).expect("Couldn't parse musicbrainz's url");
        musicbrainz_rs.musicbrainz_domain = url.domain().unwrap().to_string();
        Arc::new(musicbrainz_rs)
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

    #[cfg(feature = "musicbrainz")]
    pub fn create_symphonize_client(musicbrainz_db: Arc<DBClient>) -> Arc<SymphonyzeClient> {
        Arc::new(SymphonyzeClient {
            mb_database: musicbrainz_db,
        })
    }
}
