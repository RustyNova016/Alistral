use std::path::Path;

use google_youtube3::hyper_rustls;
use google_youtube3::hyper_rustls::HttpsConnector;
use google_youtube3::hyper_util;
use google_youtube3::hyper_util::client::legacy::connect::HttpConnector;
use google_youtube3::yup_oauth2;
use google_youtube3::YouTube;

pub struct Client {
    pub youtube_client: YoutubeClient,
}

impl Client {
    /// Create a new client
    pub async fn new(yt_secret_path: &Path, database_path: &Path) -> Result<Self, crate::Error> {
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

        Ok(Self { youtube_client })
    }


}

pub type YoutubeClient = YouTube<HttpsConnector<HttpConnector>>;