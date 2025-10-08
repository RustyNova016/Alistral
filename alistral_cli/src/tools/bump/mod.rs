use std::str::FromStr;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::Duration;
use chrono::Utc;
use clap::Parser;
use rust_decimal::Decimal;
use tuillez::extensions::chrono_exts::DurationExt as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::models::config::Config;
use crate::utils::constants::LISTENBRAINZ_FMT;
use crate::utils::user_inputs::UserInputParser;

pub mod bump_down;

/// bump a recording to show up more frequently in radios that uses scores. By default, it uses the lastest listen as target.
///
/// bump-down is an alias for `bump <RECORDING> <DURATION> 0.9`
///
/// All the bumps are added multiplicatively, so a recording won't disapear. Use the blacklist to remove them.
#[derive(Parser, Debug, Clone)]
pub struct BumpCommand {
    /// The recording to bump
    pub recording: Option<String>,

    /// The duration the bump last for (Default: 3 months)
    #[arg(short, long)]
    pub duration: Option<String>,

    /// The multiplier added to the score (Default: 1.1)
    #[arg(short, long)]
    pub multiplier: Option<String>,

    #[arg(short, long)]
    pub username: Option<String>,
}

impl BumpCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);

        let recording = match &self.recording {
            Some(mbid) => UserInputParser::parse_recording(mbid).await,
            None => {
                let user_data = ListenStatisticsData::new_from_user_listens(
                    ALISTRAL_CLIENT.core.clone(),
                    UserInputParser::username_or_default(&self.username),
                )
                .await
                .unwrap();
                user_data
                    .listens()
                    .get_latest_listen()
                    .unwrap()
                    .get_recording_or_fetch_with_task(ALISTRAL_CLIENT.musicbrainz_db.clone())
                    .await
                    .unwrap()
                    .unwrap()
            }
        };

        let multiplier = Decimal::from_str(self.multiplier.as_deref().unwrap_or("1.1"))
            .expect("Couldn't parse the multiplier");

        let duration = match &self.duration {
            Some(dur) => Duration::from_human_string(dur).expect("Couldn't parse the duration."),
            None => Duration::from_human_string("3 months").expect("Couldn't parse the duration."),
        };

        let conf = Config::load_or_panic();

        println!(
            "Adding bump to {}, giving a {} multiplier for {}",
            recording
                .format_with_async(&LISTENBRAINZ_FMT)
                .await
                .expect("Error while getting recording credits"),
            multiplier,
            duration.to_humantime().unwrap()
        );

        conf.write_or_panic().bumps.add_bump(
            recording.mbid.clone(),
            username,
            multiplier,
            Utc::now() + duration,
        );
    }
}
