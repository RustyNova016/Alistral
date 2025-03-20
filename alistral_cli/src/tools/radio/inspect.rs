use core::ops::Deref as _;
use std::collections::HashMap;

use alistral_core::cli::colors::AlistralColors;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use clap::Parser;
use futures::StreamExt as _;
use futures::TryStreamExt;
use serde_json::Value;
use tracing::debug;
use tuillez::fatal_error::IntoFatal;
use yumako_jams::json::radio::Radio;
use yumako_jams::radio_stream::RadioStreamaExt;

use crate::ALISTRAL_CLIENT;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

#[derive(Parser, Debug, Clone)]
pub struct RadioInspectCommand {
    /// The name of the radio to inspect
    radio_name: String,

    /// The recording to search
    recording: String,
}

impl RadioInspectCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let radio_schema =
            Radio::from_file(format!("./yumako_jams/exemples/{}.json", self.radio_name))
                .expect_fatal("Couldn't read the radio")?;

        let mut vars = HashMap::new();
        vars.insert(
            "username".to_string(),
            Value::String("RustyNova".to_string()),
        );
        vars.insert(
            "timeouts".to_string(),
            serde_json::to_value(load_timeouts()).unwrap(),
        );
        vars.insert(
            "listen_range".to_string(),
            Value::String("Last90Days".to_string()),
        );

        debug!("Compiling radio");
        let mut radio = match radio_schema.to_stream(&ALISTRAL_CLIENT.yumako_jams, vars) {
            Ok(val) => val,
            Err(err) => {
                compilation_error(err);
                return Ok(());
            }
        };
        debug!("Compiled radio");

        let mut i = 0;
        while let Some(track) = radio.try_next().await.unwrap() {
            i += 1;

            if track.entity().mbid == self.recording {
                println!(
                    "[#{} - {}] {}",
                    i,
                    track.score.round_sf(3).unwrap(),
                    track
                        .entity()
                        .pretty_format_with_credits(
                            &mut *conn,
                            &ALISTRAL_CLIENT.musicbrainz_db,
                            true
                        )
                        .await
                        .unwrap()
                );

                return Ok(());
            }
        }

        println!("Recording not found in the radio");

        Ok(())
    }
}

/// Read the recording timeouts
pub fn load_timeouts() -> HashMap<String, DateTime<Utc>> {
    let config = RecordingTimeoutConfig::load().expect("Couldn't fetch the timeout config");
    let config = config.read_or_panic();
    config.deref().deref().clone()
}

pub fn compilation_error(err: yumako_jams::Error) {
    println!();
    println!(
        "{}",
        "Radio compilation error".as_color_title((225, 125, 0))
    );
    println!();
    println!("{err}");
    println!();
    // #[cfg(debug_assertions)]
    // println!("{err:#?}")
}
