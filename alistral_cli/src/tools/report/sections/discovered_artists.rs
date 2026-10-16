use chrono::DateTime;
use chrono::Local;
use snafu::ResultExt;
use tuillez::pg_inc;

use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::DiscoveredArtists;
use crate::tools::report::sections::error::ArtistStatsSnafu;
use crate::tools::report::sections::error::SectionsError;

impl DiscoveredArtists {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> Result<String, SectionsError> {
        let all_stats = listens.current_stats().artists_stats().await.context(ArtistStatsSnafu)?;
        pg_inc!();
        let current_entity = listens.current_stats().artists_stats().await.context(ArtistStatsSnafu)?;
        pg_inc!();
        let previous_entity = listens.previous_stats().artists_stats().await.context(ArtistStatsSnafu)?;
        pg_inc!();

        Ok(discoveries_report(
            all_stats,
            current_entity,
            previous_entity,
            listens.start_time(),
            listens.end_time(),
            listens.previous_start_time(),
        )
        .await
        .unwrap())
    }
}
