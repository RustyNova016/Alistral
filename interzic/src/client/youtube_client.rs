use std::path::Path;
use std::sync::Arc;

use google_youtube3::YouTube;
use google_youtube3::hyper_rustls;
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::hyper_util;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::yup_oauth2;

use crate::InterzicClient;
use crate::models::services::youtube::error::InterzicYoutubeError;

pub type YoutubeClient = YouTube<HttpsConnector<HttpConnector>>;

impl InterzicClient {
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

    pub fn youtube_client(&self) -> Result<&YoutubeClient, InterzicYoutubeError> {
        self.youtube_client
            .as_ref()
            .map(Arc::as_ref)
            .ok_or(InterzicYoutubeError::MissingYoutubeClient())
    }
}
