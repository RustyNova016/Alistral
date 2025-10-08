use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Utc;
use clap::Parser;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::Timeframe;
use crate::tools::lookup::recording::printer::RecordingLookup;
use crate::utils::user_inputs::UserInputParser;

pub mod general;
pub mod printer;
pub mod rate;

#[derive(Parser, Clone, Debug)]
pub(super) struct LookupRecordingCommand {
    /// The id of the entity (Accept URLs)
    id: String,

    /// Get the data of this user
    username: Option<String>,

    /// Time period to use for the statistics.
    #[clap(short, long)]
    timeframe: Option<Timeframe>,

    /// Get statistics from this date. Use YYYY-MM-DD format
    #[clap(short, long)]
    from: Option<NaiveDate>,

    /// Get statistics until this date. Use YYYY-MM-DD format
    #[clap(short, long)]
    until: Option<NaiveDate>,
}

impl LookupRecordingCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);
        let recording = UserInputParser::parse_recording(&self.id).await;
        let stats = ALISTRAL_CLIENT.statistics_of_user(username).await;

        let lookup = match (self.from(), self.until()) {
            (Some(from), Some(until)) => {
                RecordingLookup::new_with_timeframe(stats, recording, from, until)
            }

            (Some(from), None) => {
                RecordingLookup::new_with_timeframe(stats, recording, from, Utc::now())
            }
            (None, _) => RecordingLookup::new(stats, recording),
        };

        println!("{}", lookup.print_report().await);
    }

    pub fn from(&self) -> Option<DateTime<Utc>> {
        if let Some(t) = &self.timeframe {
            return Some(t.get_start_date());
        }

        if let Some(t) = &self.from {
            //TODO: Proper error?
            return Some(
                t.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .to_utc(),
            );
        }

        None
    }

    pub fn until(&self) -> Option<DateTime<Utc>> {
        if self.timeframe.is_some() {
            return Some(Utc::now());
        }

        if let Some(t) = &self.until {
            //TODO: Proper error?
            return Some(
                t.and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Local)
                    .unwrap()
                    .to_utc(),
            );
        }

        None
    }
}
