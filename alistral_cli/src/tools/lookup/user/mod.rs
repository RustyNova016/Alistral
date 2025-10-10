use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Utc;
use clap::Parser;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::Timeframe;
use crate::tools::lookup::user::printer::UserLookup;
use crate::utils::user_inputs::UserInputParser;

pub mod printer;
#[derive(Parser, Clone, Debug)]
pub(super) struct LookupUserCommand {
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

impl LookupUserCommand {
    pub async fn run(&self) {
        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        let lookup = match (self.from(), self.until()) {
            (Some(from), Some(until)) => {
                UserLookup::new_with_timeframe(username, stats, from, until)
            }

            (Some(from), None) => UserLookup::new_with_timeframe(username, stats, from, Utc::now()),
            (None, _) => UserLookup::new(stats, username),
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
