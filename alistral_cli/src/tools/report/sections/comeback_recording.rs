use chrono::DateTime;
use chrono::Local;
use snafu::ResultExt;
use tuillez::pg_inc;

use crate::models::cli_components::reports::top_comeback::top_comeback_entities;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::ComebackArtists;
use crate::tools::report::section_types::ComebackRecordings;
use crate::tools::report::sections::error::ArtistStatsSnafu;
use crate::tools::report::sections::error::RecordingStatsSnafu;
use crate::tools::report::sections::error::SectionsError;

impl ComebackRecordings {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let all_stats = listens
            .all_time_stats()
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?;
        pg_inc!();
        let current_entity = listens
            .current_stats()
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?;
        pg_inc!();
        let previous_entity = listens
            .previous_stats()
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?;
        pg_inc!();

        Ok(top_comeback_entities(
            all_stats,
            current_entity,
            previous_entity,
            listens.time_frame(),
        )
        .await
        .unwrap())
    }
}
