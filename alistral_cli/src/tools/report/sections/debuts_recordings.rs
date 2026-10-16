use chrono::DateTime;
use chrono::Local;
use tuillez::pg_inc;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::debuts::debut_report;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::DebutsRecordings;

impl DebutsRecordings {
    pub async fn generate(
        &self,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> String {
        let all_time_entity_stats = listens.all_time_stats().recording_stats().await.unwrap();
        pg_inc!();
        let current_entity_stats = listens
            .current_all_time_stats()
            .recording_stats()
            .await
            .unwrap();
        pg_inc!();
        let previous_entity_stats = listens
            .previous_all_time_stats()
            .recording_stats()
            .await
            .unwrap();
        pg_inc!();

        debut_report(
            &ALISTRAL_CLIENT,
            &all_time_entity_stats,
            &current_entity_stats,
            &previous_entity_stats,
            start_time.to_utc(),
            end_time.to_utc(),
            previous_start_time.to_utc(),
        )
        .await
        .unwrap()
    }
}
