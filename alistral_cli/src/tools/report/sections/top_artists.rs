use chrono::DateTime;
use chrono::Local;
use snafu::ResultExt;
use tuillez::pg_inc;

use crate::models::cli_components::reports::tops::generic_report::generic_top_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::TopArtists;
use crate::tools::report::sections::error::ArtistStatsSnafu;
use crate::tools::report::sections::error::SectionsError;

impl TopArtists {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let current_entity = listens.current_stats().artists_stats().await.context(ArtistStatsSnafu)?;
        pg_inc!();
        let previous_entity = listens.previous_stats().artists_stats().await.context(ArtistStatsSnafu)?;
        pg_inc!();
        Ok(generic_top_report(current_entity, previous_entity).await)
    }
}
