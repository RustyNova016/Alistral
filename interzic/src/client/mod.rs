pub mod builder;
use std::path::Path;
use std::sync::Arc;

use google_youtube3::hyper_rustls;
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::hyper_util;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::yup_oauth2;
use google_youtube3::YouTube;
use musicbrainz_db_lite::DBClient;
use musicbrainz_rs::client::MusicBrainzClient;

use crate::client::builder::ClientBuilder;
use crate::models::services::youtube::error::YoutubeError;

pub struct InterzicClient {
    pub database_client: sqlx::SqlitePool,

    musicbrainz_client: Option<Arc<MusicBrainzClient>>,
    musicbrainz_db_lite_client: Option<Arc<DBClient>>,
    listenbrainz_client: Option<Arc<listenbrainz::raw::Client>>,
    youtube_client: Option<Arc<YoutubeClient>>,
}

impl InterzicClient {
    pub fn new_builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Create a new client
    pub async fn set_youtube_client(
        &mut self,
        yt_secret_path: &Path,
        token_cache_location: &Path,
    ) -> Result<(), crate::Error> {
        let secret = yup_oauth2::read_application_secret(yt_secret_path)
            .await
            .map_err(crate::Error::SecretFileLoadError)?;

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::Interactive,
        )
        .persist_tokens_to_disk(token_cache_location)
        .build()
        .await
        .unwrap();

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .unwrap()
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );
        let youtube_client = YouTube::new(client, auth);

        self.youtube_client = Some(Arc::new(youtube_client));
        Ok(())
    }

    pub fn youtube_client(&self) -> Result<&YoutubeClient, YoutubeError> {
        self.youtube_client
            .as_ref()
            .map(Arc::as_ref)
            .ok_or(YoutubeError::MissingYoutubeClient())
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

pub type YoutubeClient = YouTube<HttpsConnector<HttpConnector>>;
