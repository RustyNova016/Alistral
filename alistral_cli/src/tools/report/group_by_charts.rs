use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use alistral_core::models::listen_with_time::iterator::IntoListenWithDurationIterator;
use itertools::Itertools;
use tuillez::pg_inc;

use crate::models::cli_components::reports::group_by_charts::month::group_by_month_report;
use crate::tools::report::ReportCommand;

impl ReportCommand {
    pub(super) async fn group_by_reports(
        &self,
        sections: &mut Vec<String>,

        current_stats: &ListenStatisticsData,
        previous_stats: &ListenStatisticsData,
    ) {
        //let all_time_entity_stats = all_time_stats.recording_stats().await.unwrap();
        let current_entity_stats = current_stats.recording_stats().await.unwrap();
        pg_inc!();
        let previous_entity_stats = previous_stats.recording_stats().await.unwrap();
        pg_inc!();

        let current_listens = current_entity_stats
            .clone()
            .into_listen_with_duration_iterator()
            .collect_vec();
        let previous_listens = previous_entity_stats
            .clone()
            .into_listen_with_duration_iterator()
            .collect_vec();

        self.print_report(
            sections,
            group_by_month_report(&current_listens, &previous_listens),
        );
    }
}
