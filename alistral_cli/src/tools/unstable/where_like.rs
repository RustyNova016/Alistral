use core::ops::AddAssign;
use std::fs::File;
use std::io::Write;

use chrono::Local;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;

use crate::models::config::Config;
use crate::utils::constants::paths::CONFIG_DIR;

#[derive(Parser, Debug, Clone)]
pub struct WhereLikeCommand {
    username: Option<String>,
}

impl WhereLikeCommand {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let user = &Config::check_username(&self.username);
        let mut loved_recordings = Vec::new();
        let mut hated_recordings = Vec::new();
        let mut offset = 0;

        loop {

            let req = reqwest::get(format!(
            "https://api.listenbrainz.org/1/feedback/user/{user}/get-feedback?count=999&offset={offset}"
            ))
            .await
            .unwrap();

            let text = req.text().await.unwrap();
            //println!("{text}");

            let fed: FeedbackResp = serde_json::from_str(&text).unwrap();

            for feedback in fed.feedback {
                if let Some(mbid) = feedback.recording_mbid {
                    if feedback.score == 1 {
                        loved_recordings.push(mbid);
                    } else {
                        hated_recordings.push(mbid);
                    }
                }
            }

            if fed.count == 0 {
                break;
            }

            offset.add_assign(999);
        }

        Self::save_file("loved", loved_recordings);
        Self::save_file("hated", hated_recordings);

        Ok(())
    }

    fn save_file(name: &str, mut data: Vec<String>) {
        data.sort();

        let now = Local::now();

        let new_file = CONFIG_DIR.join(format!("{name}-{}", now.format("%Y-%m-%d_%H-%M-%S")));
        let mut file = File::create_new(&new_file).unwrap();

        for id in data {
            writeln!(&mut file, "{id}").unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FeedbackResp {
    count: i64,
    feedback: Vec<FeedbackItem>,
}

#[derive(Serialize, Deserialize)]
struct FeedbackItem {
    recording_mbid: Option<String>,
    score: i64,
}
