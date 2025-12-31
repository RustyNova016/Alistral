use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDateTime;
use chrono::Utc;
use clap::Parser;
use itertools::Itertools as _;
use sequelles::datastructures::ranking::Ranking;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::models::datastructures::tops::printer::top_cell::TopCell;
use crate::models::datastructures::tops::printer::top_columns::TopColumnSort;
use crate::models::datastructures::tops::printer::top_columns::TopColumnType;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::printer::top_table_printer::TopTablePrinter;
use crate::tools::stats::year_in_music::stats::YimReportData;
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

/// A recap of the year's statistics
#[derive(Parser, Debug, Clone)]
pub struct StatsYIMCommand {
    /// What year to target
    year: Option<i32>,

    /// Name of the user
    username: Option<String>,

    /// Show the listen counts of the entities as well
    #[arg(long)]
    listen_counts: bool,
}

impl StatsYIMCommand {
    pub async fn run(&self) {
        let year = self.year.unwrap_or_else(|| Utc::now().year() - 1);
        let username = UserInputParser::username_or_default(&self.username);

        println!("Welcome to your Year in Music recap!");
        println!();
        println!("Please wait while we fetch your data...");
        println!(
            "This may take a long time. Run it in the background and come back later. Progress is saved if the app is closed"
        );
        println!(
            "You can also check out options to personalise your report here: https://rustynova016.github.io/Alistral/CommandLineHelp.html#alistral-stats-yim"
        );
        println!();

        let stats = ALISTRAL_CLIENT.statistics_of_user(username).await;

        let report = YimReport::new(year, stats, self.listen_counts);
        report.data.prefetch().await;
        report.print().await;
    }
}

struct YimReport {
    year: i32,

    year_start: DateTime<Local>,
    year_end: DateTime<Local>,

    listen_counts: bool,

    data: YimReportData,
}

impl YimReport {
    pub fn new(year: i32, full_user_stats: ListenStatisticsData, listen_counts: bool) -> Self {
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

        let data = YimReportData::new(year, full_user_stats, current, previous);

        Self {
            year,
            year_end,
            year_start,
            data,
            listen_counts,
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

    pub async fn top_recordings(&self, stats: Vec<RecordingWithListens>) -> String {
        let rankings = Ranking::from(stats);
        let rankings =
            rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let rows = rankings
            .into_iter()
            .map(|(rank, rec)| TopRow {
                element: Box::new(rec.recording().clone()),
                ranking: Some(TopCell::new(Some(rank + 1), None, false)),

                listen_duration: Some(TopCell::new(
                    Some(HumanTimePrinter::from(rec.get_time_listened())),
                    None,
                    false,
                )),

                listen_count: Some(TopCell::new(Some(rec.listen_count()), None, false)),
            })
            .collect_vec();

        let table = TopTablePrinter::builder()
            .columns(self.get_top_columns())
            .sorted_column(TopColumnType::ListenDuration)
            .sort_order(TopColumnSort::Desc)
            .build();

        table.format_n_rows(rows, 20).await
    }
}
