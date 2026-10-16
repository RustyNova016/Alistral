use chrono::DateTime;
use chrono::Local;

use crate::models::cli_components::reports::group_by_charts::month::group_by_month_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::GroupByMonth;

impl GroupByMonth {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> String {
        group_by_month_report(
            listens.current_stats().listens(),
            listens.previous_stats().listens(),
        )
    }
}
