use std::path::PathBuf;
use std::sync::LazyLock;

use crate::utils::constants::CONFIG_DIR;

pub static YT_SECRET_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_credentials.json");

    path
});

pub static TOKENCACHE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_tokens.json");

    path
});

pub static INTERZIC_DB: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("symphonyz.db");

    path
});

// #[cfg(test)]
// mod test {
//     use interzic::models::messy_recording::MessyRecording;
//     use interzic::models::services::youtube::Youtube;
//     use interzic::InterzicClient;
//     use musicbrainz_db_lite::client::MusicBrainzClient;

//     use crate::api::youtube::INTERZIC_DB;
//     use crate::api::youtube::TOKENCACHE;
//     use crate::api::youtube::YT_SECRET_FILE;

//     #[tokio::test]
//     pub async fn should_request_video_id() {
//         let mut client = InterzicClient::new_builder();
//         client.set_musicbrainz_client(MusicBrainzClient::default());
//         //client.create_database_if_missing(&SYMPHONYZ_DB).unwrap();
//         //client.read_database(&SYMPHONYZ_DB.to_string_lossy()).unwrap();
//         client.read_database(":memory:").unwrap();
//         client.migrate_database().await.unwrap();
//         let mut client = client.build().unwrap();
//         client
//             .set_youtube_client(&YT_SECRET_FILE, &TOKENCACHE)
//             .await
//             .unwrap();

//         let recording = MessyRecording {
//             title: "Midnight Runners".to_string(),
//             artist_credits: "DirtyPhonics".to_string(),
//             release: Some("Magnetic".to_string()),
//             mbid: Some("77d5d71a-d7bf-4def-a105-80a6b36ac044".to_string()),
//             id: 0,
//         };
//         let recording = recording.upsert(&client.database_client).await.unwrap();

//         println!("Before send");
//         let res = Youtube::get_or_query(&client, recording, None)
//             .await
//             .unwrap()
//             .unwrap();
//         println!("after send");
//         println!("res: {res}");
//     }
// }
