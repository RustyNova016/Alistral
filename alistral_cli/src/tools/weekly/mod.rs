use chrono::Days;
use chrono::Local;
use chrono::NaiveDate;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::tools::daily::error::DailyCommandError;
use crate::utils::cli::await_next;
use crate::utils::user_inputs::UserInputParser;

pub mod error;
pub mod general_stats;

/// A combination of small statistics to run daily
#[derive(clap::Parser, Debug, Clone)]
pub struct WeeklyCommand {
    /// The date to use for the weekly report based on the previous 7 days (The day you specify doesn't count)
    date: Option<NaiveDate>,

    /// Name of the user to provide a daily report
    #[arg(short, long)]
    username: Option<String>,
}

impl WeeklyCommand {
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

        let start_time = (today - Days::new(7)).to_utc();
        let end_time = today.to_utc();
        let previous_start_time = (start_time - Days::new(7)).to_utc();

        let (current, old) = stats.comparison_split(start_time, end_time);

        self.print_report(general_stats_report(&current, &old, start_time, end_time).await.unwrap());
        self.print_report(
            discoveries_report(
                &stats,
                &current,
                &old,
                start_time,
                end_time,
                previous_start_time,
            )
            .await
            .unwrap(),
        );

        Ok(())
    }

    fn print_report(&self, data: String) {
        println!("{data}");
        await_next();
    }
}
