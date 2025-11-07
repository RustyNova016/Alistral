use std::fmt::Write;

use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDateTime;
use clap::Parser;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli::await_next;
use crate::utils::user_inputs::UserInputParser;

pub mod new_releases;
pub mod random_stats;
pub mod recordings;

/// A recap of the year's statistics
#[derive(Parser, Debug, Clone)]
pub struct StatsYIMCommand {
    /// What year to target
    year: Option<i32>,

    /// Name of the user
    username: Option<String>,
}

impl StatsYIMCommand {
    pub async fn run(&self) {
        let year = self.year.unwrap_or(2024);
        let username = UserInputParser::username_or_default(&self.username);

        println!("Welcome to your Year in Music recap!");
        println!();
        println!("Please wait while we fetch your data...");

        let stats = ALISTRAL_CLIENT.statistics_of_user(username).await;

        let report = YimReport::new(year, stats);
        report.print().await;
    }
}

struct YimReport {
    year: i32,

    year_start: DateTime<Local>,
    year_end: DateTime<Local>,

    current: ListenStatisticsData,
    previous: ListenStatisticsData,
}

impl YimReport {
    pub fn new(year: i32, stats: ListenStatisticsData) -> Self {
        let year_start =
            NaiveDateTime::parse_from_str(&format!("{}-01-01 00:00:00", year), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_local_timezone(Local)
                .earliest()
                .unwrap();
        let year_end = year_start.with_year(year + 1).unwrap();
        let previous_year = year_start.with_year(year - 1).unwrap();

        let current = stats
            .clone_no_stats()
            .filter_listening_date(year_start.into(), year_end.into());

        let previous = stats.filter_listening_date(previous_year.into(), year_start.into());

        Self {
            current,
            previous,
            year,
            year_end,
            year_start,
        }
    }

    pub async fn print(&self) {
        println!("{}", self.random_stats_report().await);
        println!("[Press enter to continue]");
        await_next();
        println!("{}", self.recording_report().await);
        println!("[Press enter to continue]");
        await_next();
    }
}
