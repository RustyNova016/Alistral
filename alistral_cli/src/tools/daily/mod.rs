use chrono::Local;
use chrono::NaiveDate;
use clap::Parser;

use crate::ALISTRAL_CLIENT;
use crate::utils::user_inputs::UserInputParser;

pub mod first_discoveries;
pub mod fresh_releases;
pub mod recording_birthdays;

/// A combination of small statistics to run daily
#[derive(Parser, Debug, Clone)]
pub struct DailyCommand {
    /// The date to use for the daily report.
    date: Option<NaiveDate>,

    /// Name of the user to provide a daily report
    username: Option<String>,
}

impl DailyCommand {
    pub async fn run(&self) {
        let today = UserInputParser::parse_naive_date(self.date).unwrap_or(Local::now());
        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        Self::print_recording_birthdays(&stats, today).await;
        Self::print_first_discoveries(&stats, today).await;
        Self::print_fresh_releases(&stats, today).await;
    }
}
