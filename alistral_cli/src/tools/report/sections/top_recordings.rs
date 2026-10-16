use chrono::DateTime;
use chrono::Local;
use snafu::ResultExt;
use tuillez::pg_inc;

use crate::models::cli_components::reports::tops::generic_report::generic_top_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::TopRecordings;
use crate::tools::report::sections::error::RecordingStatsSnafu;
use crate::tools::report::sections::error::SectionsError;

impl TopRecordings {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let current_entity = listens.current_stats().recording_stats().await.context(RecordingStatsSnafu)?;
        pg_inc!();
        let previous_entity = listens.previous_stats().recording_stats().await.context(RecordingStatsSnafu)?;
        pg_inc!();
        Ok(generic_top_report(current_entity, previous_entity).await)
    }
}
