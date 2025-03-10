use core::ops::Deref as _;
use std::collections::HashMap;

use chrono::DateTime;
use chrono::Utc;
use clap::Parser;
use futures::TryStreamExt;
use serde_json::Value;
use tracing::debug;
use tuillez::fatal_error::IntoFatal;
use yumako_jams::json::radio::Radio;

use crate::ALISTRAL_CLIENT;
use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;

#[derive(Parser, Debug, Clone)]
pub struct RadioYumakoCommand {}

impl RadioYumakoCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let radio_schema = Radio::from_file("./yumako_jams/exemples/listenrate_radio.json")
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

        debug!("Compiling radio");
        let mut radio = radio_schema
            .to_stream(&ALISTRAL_CLIENT.yumako_jams, vars)
            .unwrap();
        debug!("Compiled radio");

        for _ in 0..20 {
            let track = radio.try_next().await.unwrap().unwrap();

            println!(
                "{}",
                track
                    .entity()
                    .pretty_format_with_credits(conn, &ALISTRAL_CLIENT.musicbrainz_db, true)
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
