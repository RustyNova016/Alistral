use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Local;

use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::models::cli_components::reports::tops::generic_report::generic_top_report;
use crate::tools::report::ReportCommand;

impl ReportCommand {
    pub(super) async fn recording_reports(
        &self,
        sections: &mut Vec<String>,

        all_time_stats: &ListenStatisticsData,
        current_stats: &ListenStatisticsData,
        previous_stats: &ListenStatisticsData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) {
        let all_time_entity_stats = all_time_stats.recording_stats().await.unwrap();
        let current_entity_stats = current_stats.recording_stats().await.unwrap();
        let previous_entity_stats = previous_stats.recording_stats().await.unwrap();

        self.print_report(
            sections,
            generic_top_report(current_entity_stats, previous_entity_stats).await,
        );

        self.print_report(
            sections,
            discoveries_report(
                &all_time_entity_stats,
                &current_entity_stats,
                &previous_entity_stats,

                start_time.to_utc(),
                end_time.to_utc(),
                previous_start_time.to_utc(),
            )
            .await
            .unwrap(),
        );
    }
}
