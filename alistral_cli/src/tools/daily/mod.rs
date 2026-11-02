use chrono::Local;
use chrono::NaiveDate;

use crate::ALISTRAL_CLIENT;
use crate::tools::daily::daily_stats::daily_stats;
use crate::tools::daily::error::DailyCommandError;
use crate::utils::user_inputs::UserInputParser;

pub mod daily_stats;
pub mod error;
pub mod first_discoveries;
pub mod fresh_releases;
pub mod recording_birthdays;

/// A combination of small statistics to run daily
#[derive(clap::Parser, Debug, Clone)]
pub struct DailyCommand {
    /// The date to use for the daily report.
    date: Option<NaiveDate>,

    /// Name of the user to provide a daily report
    username: Option<String>,
}

impl DailyCommand {
    pub async fn run(&self) -> Result<(), DailyCommandError> {
        let today = UserInputParser::parse_naive_date(self.date).unwrap_or(
            Local::now()
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Local)
                .unwrap(),
        );
        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        daily_stats(&stats, today).await;
        Self::print_recording_birthdays(&ALISTRAL_CLIENT, &stats, today).await;
        Self::print_first_discoveries(&ALISTRAL_CLIENT, &stats, today).await?;
        Self::print_fresh_releases(&stats, today).await;

        Ok(())
    }
}
