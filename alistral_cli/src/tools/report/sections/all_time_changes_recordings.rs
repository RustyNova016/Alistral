use chrono::DateTime;
use chrono::Local;
use snafu::ResultExt;
use tuillez::pg_inc;

use crate::models::cli_components::reports::all_time_changes::all_time_changes;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::AllTimeChangesRecordings;
use crate::tools::report::sections::error::RecordingStatsSnafu;
use crate::tools::report::sections::error::SectionsError;

impl AllTimeChangesRecordings {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
       _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let current_entity_all_time = listens
            .current_all_time_stats()
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?;
        pg_inc!();
        let previous_entity_all_time = listens
            .previous_all_time_stats()
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?;
        pg_inc!();
        Ok(all_time_changes(current_entity_all_time, previous_entity_all_time).await)
    }
}
