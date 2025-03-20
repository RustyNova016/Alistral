use core::ops::Deref as _;
use std::collections::HashMap;

use alistral_core::cli::colors::AlistralColors;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use clap::Parser;
use tracing::debug;
use tuillez::fatal_error::IntoFatal;
use yumako_jams::json::radio::Radio;
use yumako_jams::radio_stream::RadioStreamaExt;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::parsers::yumako_parser::parse_yumako_variables;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

#[derive(Parser, Debug, Clone)]
pub struct RadioYumakoCommand {
    /// The name of the radio to inspect
    radio_name: String,

    /// Radio arguments
    arguments: Vec<String>,
}

impl RadioYumakoCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let mut args = parse_yumako_variables(&self.arguments.join(" ")).unwrap();

        let radio_schema =
            Radio::from_file(format!("./yumako_jams/exemples/{}.json", self.radio_name))
                .expect_fatal("Couldn't read the radio")?;

        args.insert(
            "timeouts".to_string(),
            serde_json::to_value(load_timeouts()).unwrap(),
        );
        // vars.insert(
        //     "listen_range".to_string(),
        //     Value::String("Last90Days".to_string()),
        // );

        debug!("Compiling radio");
        let radio = match radio_schema.to_stream(&ALISTRAL_CLIENT.yumako_jams, args) {
            Ok(val) => val,
            Err(err) => {
                compilation_error(err);
                return Ok(());
            }
        };
        debug!("Compiled radio");

        for track in radio.collect_with(50, Duration::zero()).await {
            let track = track.unwrap();

            println!(
                "[{}] {}",
                track.score.round_sf(3).unwrap(),
                track
                    .entity()
                    .pretty_format_with_credits(&mut *conn, &ALISTRAL_CLIENT.musicbrainz_db, true)
                    .await
                    .unwrap()
            )
        }

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
