pub mod builder;
use std::path::Path;

use google_youtube3::hyper_rustls;
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::hyper_util;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::yup_oauth2;
use google_youtube3::YouTube;
use musicbrainz_rs::client::MusicBrainzClient;

use crate::client::builder::ClientBuilder;

pub struct Client {
    pub database_client: sqlx::SqlitePool,
    pub musicbrainz_client: MusicBrainzClient,
    youtube_client: Option<YoutubeClient>,
}

impl Client {
    pub fn new_builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Create a new client
    pub async fn set_youtube_client(&mut self, yt_secret_path: &Path) -> Result<(), crate::Error> {
        let secret = yup_oauth2::read_application_secret(yt_secret_path)
            .await
            .map_err(crate::Error::SecretFileLoadError)?;

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::Interactive,
        )
        .persist_tokens_to_disk("tokencache.json")
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

        self.youtube_client = Some(youtube_client);
        Ok(())
    }

    pub fn youtube_client(&self) -> Result<&YoutubeClient, crate::Error> {
        self.youtube_client
            .as_ref()
            .ok_or(crate::Error::MissingYoutubeClient())
    }
}

pub type YoutubeClient = YouTube<HttpsConnector<HttpConnector>>;
