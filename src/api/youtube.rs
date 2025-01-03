use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;

use once_cell::sync::Lazy;

use crate::utils::constants::CONFIG_DIR;

pub static YT_SECRET_FILE: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_credentials.json");

    println!("path: {}", path.display());
    path
});

#[cfg(test)]
mod test {
    use std::path::Path;

    use youtube_brainz::models::messy_recording::MessyRecording;
    use youtube_brainz::Client;

    use crate::api::youtube::YT_SECRET_FILE;
    use crate::utils::constants::CONFIG_DIR;

    #[tokio::test]
    pub async fn should_request_video_id() {
        let client = Client::new(&YT_SECRET_FILE, &CONFIG_DIR).await.unwrap();
        let recording = MessyRecording {
            title: "Midnight Runners".to_string(),
            artist_credits: "DirtyPhonics".to_string(),
            release: "Magnetic".to_string(),
        };

        println!("Before send");
        let res = client
            .get_recording_yt_id(recording)
            .await
            .unwrap()
            .unwrap();
        println!("after send");
        println!("res: {res}");
    }
}
