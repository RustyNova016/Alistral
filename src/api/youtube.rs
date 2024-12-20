use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;
use google_youtube3::yup_oauth2;
use google_youtube3::yup_oauth2::ApplicationSecret;
use google_youtube3::yup_oauth2::ConsoleApplicationSecret;
use once_cell::sync::Lazy;

use crate::utils::constants::CONFIG_DIR;

pub static YT_SECRET_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_credentials.json");

    path
});

pub async fn get_youtube_auth() -> Result<(), crate::Error> {
    let secret = yup_oauth2::read_application_secret(YT_SECRET_FILE).await?;

    yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .build()
    .await
    .unwrap();

    Ok(())
}
