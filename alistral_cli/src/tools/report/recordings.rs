use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Local;
use tuillez::pg_inc;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::all_time_changes::all_time_changes;
use crate::models::cli_components::reports::debuts::debut_report;
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

        current_all_time: &ListenStatisticsData,
        previous_all_time: &ListenStatisticsData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) {
        let all_time_entity_stats = all_time_stats.recording_stats().await.unwrap();
        pg_inc!();
        let current_entity_stats = current_stats.recording_stats().await.unwrap();
        pg_inc!();
        let previous_entity_stats = previous_stats.recording_stats().await.unwrap();
        pg_inc!();
        let current_entity_all_time = current_all_time.recording_stats().await.unwrap();
        pg_inc!();
        let previous_entity_all_time = previous_all_time.recording_stats().await.unwrap();
        pg_inc!();

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

        self.print_report(
            sections,
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
            .unwrap(),
        );

        self.print_report(
            sections,
            all_time_changes(current_entity_all_time, previous_entity_all_time).await,
        );
    }
}
