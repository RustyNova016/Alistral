use chrono::DateTime;
use chrono::Local;
use tuillez::pg_inc;

use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::GeneralStats;
use crate::tools::report::sections::error::SectionsError;

impl GeneralStats {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let text = general_stats_report(
            listens.current_stats(),
            listens.previous_stats(),
            start_time,
            end_time,
        )
        .await
        .unwrap();

        pg_inc!();

        Ok(text)
    }

    pub fn loading_steps(&self) -> u64 {
        1
    }
}
