pub mod section_list;
use chrono::DateTime;
use chrono::Local;
use chrono::NaiveDate;
use tracing::Instrument;
use tracing::info_span;
use tracing_indicatif::span_ext::IndicatifSpanExt;
use tuillez::pg_inc;

use crate::ALISTRAL_CLIENT;
use crate::tools::report::error::ReportCommandError;
use crate::tools::report::listen_data::ReportListenData;
use crate::tools::report::section_types::SectionType;
use crate::tools::report::section_types::SectionTypeTrait;
use crate::utils::cli::await_next;
use crate::utils::user_inputs::UserInputParser;

pub mod error;
pub mod listen_data;
pub mod section_types;
pub mod sections;

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
    pub async fn run(&self) -> Result<(), ReportCommandError> {
        let start_time = UserInputParser::parse_naive_date(Some(self.start)).unwrap();
        let end_time = UserInputParser::parse_naive_date(Some(self.end)).unwrap();
        let period_duration = end_time - start_time;
        let previous_start_time = start_time - period_duration;

        // === Create report ===
        let span = if !self.lazy_fetching {
            println!("Welcome to your Alistral report!");
            println!();
            println!(
                "We are currently fetching and compiling your data. This may take a long time, so you can run it in the background and come check on it later. Progress is saved if the app is closed"
            );
            println!(
                "If you are willing to wait between sections, you can rerun you command with the --lazy-fetching option"
            );
            println!();

            let span = info_span!("report", indicatif.pb_show = tracing::field::Empty);
            span.pb_start();
            span.pb_set_length(32);
            span.pb_set_message("Creating your report");
            span
        } else {
            info_span!("report")
        };

        let sections = self
            .create2(start_time, end_time, previous_start_time)
            .instrument(span)
            .await
            .unwrap();

        for section in sections {
            println!("{section}");

            if !self.no_wait {
                println!("<Press enter to continue>");
                await_next();
            }
        }

        Ok(())
    }

    async fn create2(
        &self,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> Result<Vec<String>, ()> {
        let mut sections = Vec::with_capacity(20);
        let username = UserInputParser::username_or_default(&self.username);
        let all_time_stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        let listens = ReportListenData::new(
            start_time.to_utc(),
            end_time.to_utc(),
            previous_start_time.to_utc(),
            all_time_stats,
        );

        let todo = [
            SectionType::GeneralStats(Default::default()),
            SectionType::TopRecordings(Default::default()),
            SectionType::DiscoveredRecordings(Default::default()),
            SectionType::DebutsRecordings(Default::default()),
            SectionType::AllTimeChangesRecordings(Default::default()),
            SectionType::TopArtists(Default::default()),
            SectionType::DiscoveredArtists(Default::default()),
            SectionType::AllTimeChangesArtists(Default::default()),
            SectionType::TopReleases(Default::default()),
            SectionType::DiscoveredReleases(Default::default()),
            SectionType::DebutsReleases(Default::default()),
            SectionType::AllTimeChangesReleases(Default::default()),
            SectionType::TopReleaseGroups(Default::default()),
            SectionType::DiscoveredReleaseGroups(Default::default()),
            SectionType::AllTimeChangesReleaseGroups(Default::default()),
            SectionType::Toplabels(Default::default()),
            SectionType::DiscoveredLabels(Default::default()),
            SectionType::AllTimeChangesLabels(Default::default()),
            SectionType::GroupByWeekDay(Default::default()),
            SectionType::GroupByMonth(Default::default()),
        ];

        for section_type in todo {
            self.print_report(
                &mut sections,
                section_type.generate(
                    &listens,
                    start_time,
                    end_time,
                    previous_start_time,
                )
                .await,
            );
        }

        Ok(sections)
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
