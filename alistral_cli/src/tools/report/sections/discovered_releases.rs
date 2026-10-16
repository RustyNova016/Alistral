use chrono::DateTime;
use chrono::Local;
use tuillez::pg_inc;

use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::DiscoveredReleases;

impl DiscoveredReleases {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        _start_time: DateTime<Local>,
        _end_time: DateTime<Local>,
        _previous_start_time: DateTime<Local>,
    ) -> String {
        let all_stats = listens.current_stats().release_stats().await.unwrap();
        pg_inc!();
        let current_entity = listens.current_stats().release_stats().await.unwrap();
        pg_inc!();
        let previous_entity = listens.previous_stats().release_stats().await.unwrap();
        pg_inc!();

        discoveries_report(
            all_stats,
            current_entity,
            previous_entity,
            listens.start_time(),
            listens.end_time(),
            listens.previous_start_time(),
        )
        .await
        .unwrap()
    }
}
