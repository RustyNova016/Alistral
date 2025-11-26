use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDateTime;
use clap::Parser;
use itertools::Itertools as _;
use sequelles::datastructures::ranking::Ranking;

use crate::ALISTRAL_CLIENT;
use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::utils::cli::await_next;
use crate::utils::user_inputs::UserInputParser;

pub mod artists;
pub mod components;
pub mod discoveries;
pub mod label_stats;
pub mod month_lookup;
pub mod month_recap;
pub mod new_releases;
pub mod random_stats;
pub mod recordings;
pub mod releases;
pub mod stats;
pub mod tags;



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
        println!("(This may take a long time. Run it in the background and come back later. Progress is saved if the app is closed)");

        let stats = ALISTRAL_CLIENT.statistics_of_user(username).await;

        let report = YimReport::new(year, stats);
        report.prefetch_data().await;
        report.print().await;
    }
}

struct YimReport {
    year: i32,

    year_start: DateTime<Local>,
    year_end: DateTime<Local>,

    /// All the user data
    full_user_stats: ListenStatisticsData,

    current: ListenStatisticsData,
    previous: ListenStatisticsData,
}

impl YimReport {
    pub fn new(year: i32, full_user_stats: ListenStatisticsData) -> Self {
        let year_start =
            NaiveDateTime::parse_from_str(&format!("{}-01-01 00:00:00", year), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .and_local_timezone(Local)
                .earliest()
                .unwrap();
        let year_end = year_start.with_year(year + 1).unwrap();
        let previous_year = year_start.with_year(year - 1).unwrap();

        let current = full_user_stats
            .clone_no_stats()
            .filter_listening_date(year_start.into(), year_end.into());

        let previous = full_user_stats
            .clone_no_stats()
            .filter_listening_date(previous_year.into(), year_start.into());

        Self {
            current,
            previous,
            year,
            year_end,
            year_start,
            full_user_stats,
        }
    }

    pub async fn print(&self) {
        println!("{}", self.random_stats_report().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.recording_report().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.new_release_page().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.discoveries_page().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.artist_report().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.release_report().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.labels_report().await);
        println!("[Press enter to continue]");
        await_next();

        println!("{}", self.monthly_recap_page().await);
        println!("[Press enter to continue]");
        await_next();

        self.all_month_lookup_page().await;

        await_next();
        println!("That's all folks! See you next year (Or anytime you want)!");
    }

    pub async fn top_recordings(stats: Vec<RecordingWithListens>) -> String {
        let rankings = Ranking::from(stats);
        let rankings =
            rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let rows = rankings
            .into_iter()
            .map(|(rank, rec)| TopRow {
                ranking: rank + 1,
                score: TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default()),
                element: Box::new(rec.recording().clone()),
                previous_ranking: None,
                previous_score: None,
            })
            .collect_vec();

        TopPrinter::format_n_rows(rows, 20).await
    }
}



