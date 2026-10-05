use chrono::NaiveDate;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::models::cli_components::reports::tops::top_artists_report;
use crate::models::cli_components::reports::tops::top_label_report;
use crate::models::cli_components::reports::tops::top_recordings_report;
use crate::models::cli_components::reports::tops::top_releases_group_report;
use crate::models::cli_components::reports::tops::top_releases_report;
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

    /// Enable lazy fetching. Instead of calculating all the stats at once, this calculate the stats when needed to display the next section.
    /// This allows progressively seeing the data without waiting for everything to compile
    #[arg(long)]
    lazy_fetching: bool,

    /// Disable the "Press enter to continue prompts"
    #[arg(long)]
    no_wait: bool,
}

impl ReportCommand {
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn run(&self) -> Result<(), ReportCommandError> {
        let start_time = UserInputParser::parse_naive_date(Some(self.start)).unwrap();
        let end_time = UserInputParser::parse_naive_date(Some(self.end)).unwrap();
        let period_duration = end_time - start_time;
        let previous_start_time = (start_time - period_duration).to_utc();

        let username = UserInputParser::username_or_default(&self.username);
        let stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        let (current_stats, previous_stats) =
            stats.comparison_split(start_time.to_utc(), end_time.to_utc());

        // === Create report ===
        let mut sections = Vec::with_capacity(20);

        if !self.lazy_fetching {
            println!("Welcome to your Alistral report!");
            println!();
            println!(
                "We are currently fetching and compiling your data. This may take a long time, so you can run it in the background and come check on it later. Progress is saved if the app is closed"
            );
            println!(
                "If you are willing to wait between sections, you can rerun you command with the --lazy-fetching option"
            );
            println!();
            pg_counted!(7, "Creating your report");
        }

        self.print_report(
            &mut sections,
            general_stats_report(&current_stats, &previous_stats, start_time, end_time)
                .await
                .unwrap(),
        );

        self.print_report(
            &mut sections,
            top_recordings_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );

        self.print_report(
            &mut sections,
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

        self.print_report(
            &mut sections,
            top_artists_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );

        self.print_report(
            &mut sections,
            top_releases_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );

        self.print_report(
            &mut sections,
            top_releases_group_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );

        self.print_report(
            &mut sections,
            top_label_report(&current_stats, &previous_stats)
                .await
                .unwrap(),
        );

        for section in sections {
            println!("{section}");

            if !self.no_wait {
                println!("<Press enter to continue>");
                await_next();
            }
        }

        Ok(())
    }

    fn print_report(&self, sections: &mut Vec<String>, data: String) {
        if !self.lazy_fetching {
            sections.push(data);
            pg_inc!();
            return;
        }

        // Lazy fetching! We print immediatly
        println!("{data}");

        if self.no_wait {
            return;
        }

        println!("<Press enter to continue>");
        await_next();
    }
}
