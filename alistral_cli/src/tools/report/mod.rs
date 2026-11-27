use alien_time::parse_human_interval;
use chrono::DateTime;
use chrono::Local;
use jiff::ZonedDateTime;
use jiff_chrono_conversions::ToChrono as _;
use snafu::OptionExt as _;
use snafu::ResultExt as _;
use tracing::Instrument as _;
use tracing::info_span;
use tracing_indicatif::span_ext::IndicatifSpanExt as _;
use tuillez::pg_inc;

use crate::ALISTRAL_CLIENT;
use crate::models::config::commands::reports::ReportConfig;
use crate::models::config::commands::reports::ReportPreset;
use crate::tools::report::error::IntervalSnafu;
use crate::tools::report::error::ReportCommandError;
use crate::tools::report::error::SectionsSnafu;
use crate::tools::report::error::UnknownPresetSnafu;
use crate::tools::report::listen_data::ReportListenData;
use crate::tools::report::section_types::SectionTypeTrait as _;
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
    time_interval: String,

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

    /// Use this display preset
    #[arg(long)]
    preset: Option<String>,
}

impl ReportCommand {
    pub async fn run(&self) -> Result<(), ReportCommandError> {
        let (start_time, end_time) =
            parse_human_interval(&self.time_interval, ZonedDateTime::now()).context(
                IntervalSnafu {
                    interval: self.time_interval.to_owned(),
                },
            )?;

        let start_time = start_time.to_chrono().with_timezone(&Local);
        let end_time = end_time.to_chrono().with_timezone(&Local);
        let period_duration = end_time - start_time;
        let previous_start_time = start_time - period_duration;

        let preset = self.get_preset()?;

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
            span.pb_set_length(preset.sections.iter().map(|sec| sec.loading_steps()).sum());
            span.pb_set_message("Creating your report");
            span
        } else {
            info_span!("report")
        };

        let sections = self
            .create(start_time, end_time, previous_start_time, &preset)
            .instrument(span)
            .await?;

        for section in sections {
            println!("{section}");

            if !self.no_wait {
                println!("<Press enter to continue>");
                await_next();
            }
        }

        Ok(())
    }

    fn get_preset(&self) -> Result<ReportPreset, ReportCommandError> {
        match &self.preset {
            None => Ok(ReportConfig::get_default_preset()),
            Some(preset) => ALISTRAL_CLIENT
                .config
                .commands
                .as_ref()
                .and_then(|cmds| cmds.reports())
                .and_then(|report_conf| report_conf.get_preset(preset))
                .cloned()
                .context(UnknownPresetSnafu {
                    preset: preset.to_string(),
                }),
        }
    }

    async fn create(
        &self,
        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
        preset: &ReportPreset,
    ) -> Result<Vec<String>, ReportCommandError> {
        let mut sections = Vec::with_capacity(20);
        let username = UserInputParser::username_or_default(&self.username);
        let all_time_stats = ALISTRAL_CLIENT.statistics_of_user(username.clone()).await;

        let listens = ReportListenData::new(
            start_time.to_utc(),
            end_time.to_utc(),
            previous_start_time.to_utc(),
            all_time_stats,
        );

        for section_type in &preset.sections {
            self.print_report(
                &mut sections,
                section_type
                    .generate(&listens, start_time, end_time, previous_start_time)
                    .await
                    .context(SectionsSnafu)?,
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
