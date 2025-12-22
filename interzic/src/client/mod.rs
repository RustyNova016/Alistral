use std::sync::Arc;

use musicbrainz_db_lite::DBClient;
use musicbrainz_rs::client::MusicBrainzClient;

use crate::client::builder::ClientBuilder;
#[cfg(feature = "youtube")]
use crate::client::youtube_client::YoutubeClient;

pub mod builder;
#[cfg(feature = "youtube")]
pub mod youtube_client;


pub struct InterzicClient {
    pub database_client: sqlx::SqlitePool,

    musicbrainz_client: Option<Arc<MusicBrainzClient>>,
    musicbrainz_db_lite_client: Option<Arc<DBClient>>,
    listenbrainz_client: Option<Arc<listenbrainz::raw::Client>>,

    #[cfg(feature = "youtube")]
    youtube_client: Option<Arc<YoutubeClient>>,
}

impl InterzicClient {
    pub fn new_builder() -> ClientBuilder {
        ClientBuilder::default()
    }


    pub fn set_listenbrainz_client(&mut self, client: Arc<listenbrainz::raw::Client>) {
        self.listenbrainz_client = Some(client);
    }

    pub fn listenbrainz_client(&self) -> Result<&listenbrainz::raw::Client, crate::Error> {
        self.listenbrainz_client
            .as_ref()
            .map(Arc::as_ref)
            .ok_or(crate::Error::MissingListenbrainzClient())
    }

    pub fn set_musicbrainz_db_lite_client(&mut self, client: Arc<DBClient>) {
        self.musicbrainz_db_lite_client = Some(client);
    }

    pub fn musicbrainz_db_lite_client(&self) -> Result<&DBClient, crate::Error> {
        self.musicbrainz_db_lite_client
            .as_ref()
            .map(Arc::as_ref)
            .ok_or(crate::Error::MissingMusicbrainzDbLiteClient)
    }

    pub fn set_musicbrainz_client(&mut self, client: Arc<MusicBrainzClient>) {
        self.musicbrainz_client = Some(client)
    }

    pub fn musicbrainz_rs_client(&self) -> Result<&MusicBrainzClient, crate::Error> {
        self.musicbrainz_client
            .as_ref()
            .map(Arc::as_ref)
            .ok_or(crate::Error::MissingMusicbrainzClient)
    }
}



