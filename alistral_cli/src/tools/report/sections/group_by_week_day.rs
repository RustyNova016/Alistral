use chrono::DateTime;
use chrono::Local;

use crate::models::cli_components::reports::group_by_charts::days::group_by_day_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::GroupByWeekDay;
use crate::tools::report::sections::error::SectionsError;

impl GroupByWeekDay {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        Ok(group_by_day_report(
            listens.current_stats().listens(),
            listens.previous_stats().listens(),
        ))
    }
}
