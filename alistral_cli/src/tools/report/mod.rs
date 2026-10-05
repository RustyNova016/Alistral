use chrono::NaiveDate;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::models::cli_components::reports::recording_top::top_recordings_report;
use crate::tools::report::error::ReportCommandError;
use crate::utils::cli::await_next;
use crate::utils::user_inputs::UserInputParser;

pub mod error;

/// A report full of stats on a given timeframe
#[derive(clap::Parser, Debug, Clone)]
pub struct ReportCommand {
    /// The date to use for the weekly report based on the previous 7 days (The day you specify doesn't count)
    start: NaiveDate,

    end: NaiveDate,

    /// Name of the user to provide a daily report
    #[arg(short, long)]
    username: Option<String>,
}

impl ReportCommand {
    pub async fn run(&self) -> Result<(), ReportCommandError> {
        let start_time = UserInputParser::parse_naive_date(Some(self.start))
            .unwrap();
        let end_time = UserInputParser::parse_naive_date(Some(self.end))
            .unwrap();
        let period_duration = end_time - start_time;
        let previous_start_time = (start_time - period_duration).to_utc();

        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        let (current_stats, previous_stats) = stats.comparison_split(start_time.to_utc(), end_time.to_utc());

        self.print_report(
            general_stats_report(&current_stats, &previous_stats, start_time, end_time)
                .await
                .unwrap(),
        );
        self.print_report(
            top_recordings_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );
        self.print_report(
            discoveries_report(
                &stats,
                &current_stats,
                &previous_stats,
                start_time.to_utc(),
                end_time.to_utc(),
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
