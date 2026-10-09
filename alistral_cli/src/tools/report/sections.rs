use chrono::DateTime;
use chrono::Local;

use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::models::cli_components::reports::group_by_charts::days::group_by_day_report;
use crate::models::cli_components::reports::group_by_charts::month::group_by_month_report;
use crate::tools::report::ReportCommand;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::SectionType;

impl ReportCommand {
    pub async fn generate_section(
        &self,
        section_type: SectionType,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> String {
        match section_type {
            SectionType::GeneralStats => general_stats_report(
                listens.current_stats(start_time.to_utc(), end_time.to_utc()),
                listens.previous_stats(previous_start_time.to_utc(), start_time.to_utc()),
                start_time,
                end_time,
            )
            .await
            .unwrap(),

            SectionType::GroupByWeekDay => group_by_day_report(
                listens
                    .current_stats(start_time.to_utc(), end_time.to_utc())
                    .listens(),
                listens
                    .previous_stats(previous_start_time.to_utc(), start_time.to_utc())
                    .listens(),
            ),

            SectionType::GroupByMonth => group_by_month_report(
                listens
                    .current_stats(start_time.to_utc(), end_time.to_utc())
                    .listens(),
                listens
                    .previous_stats(previous_start_time.to_utc(), start_time.to_utc())
                    .listens(),
            ),
        }
    }
}
