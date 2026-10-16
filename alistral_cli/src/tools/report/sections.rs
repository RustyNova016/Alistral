use chrono::DateTime;
use chrono::Local;

use crate::ALISTRAL_CLIENT;
use crate::models::cli_components::reports::all_time_changes::all_time_changes;
use crate::models::cli_components::reports::debuts::debut_report;
use crate::models::cli_components::reports::general_report::general_stats_report;
use crate::models::cli_components::reports::group_by_charts::days::group_by_day_report;
use crate::models::cli_components::reports::group_by_charts::month::group_by_month_report;
use crate::models::cli_components::reports::new_discoveries::discoveries_report;
use crate::models::cli_components::reports::tops::generic_report::generic_top_report;
use crate::tools::report::ReportCommand;
use crate::tools::report::ReportListenData;
use crate::tools::report::section_types::SectionType;

impl ReportCommand {
    pub(super) async fn generate_section(
        &self,
        section_type: SectionType,
        listens: &ReportListenData,

        start_time: DateTime<Local>,
        end_time: DateTime<Local>,
        previous_start_time: DateTime<Local>,
    ) -> String {
        match section_type {
            SectionType::GeneralStats => general_stats_report(
                listens.current_stats(),
                listens.previous_stats(),
                start_time,
                end_time,
            )
            .await
            .unwrap(),

            SectionType::GroupByWeekDay => group_by_day_report(
                listens.current_stats().listens(),
                listens.previous_stats().listens(),
            ),

            SectionType::GroupByMonth => group_by_month_report(
                listens.current_stats().listens(),
                listens.previous_stats().listens(),
            ),

            SectionType::TopArtists => {
                let current_entity = listens.current_stats().artists_stats().await.unwrap();
                let previous_entity = listens.previous_stats().artists_stats().await.unwrap();
                generic_top_report(current_entity, previous_entity).await
            }

            SectionType::Toplabels => {
                let current_entity = listens.current_stats().label_stats().await.unwrap();
                let previous_entity = listens.previous_stats().label_stats().await.unwrap();
                generic_top_report(current_entity, previous_entity).await
            }

            SectionType::TopRecordings => {
                let current_entity = listens.current_stats().recording_stats().await.unwrap();
                let previous_entity = listens.previous_stats().recording_stats().await.unwrap();
                generic_top_report(current_entity, previous_entity).await
            }

            SectionType::TopReleases => {
                let current_entity = listens.current_stats().release_stats().await.unwrap();
                let previous_entity = listens.previous_stats().release_stats().await.unwrap();
                generic_top_report(current_entity, previous_entity).await
            }

            SectionType::TopReleaseGroups => {
                let current_entity = listens.current_stats().release_group_stats().await.unwrap();
                let previous_entity = listens
                    .previous_stats()
                    .release_group_stats()
                    .await
                    .unwrap();
                generic_top_report(current_entity, previous_entity).await
            }

            SectionType::DiscoveredArtists => {
                let all_stats = listens.current_stats().artists_stats().await.unwrap();
                let current_entity = listens.current_stats().artists_stats().await.unwrap();
                let previous_entity = listens.previous_stats().artists_stats().await.unwrap();

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

            SectionType::DiscoveredLabels => {
                let all_stats = listens.current_stats().label_stats().await.unwrap();
                let current_entity = listens.current_stats().label_stats().await.unwrap();
                let previous_entity = listens.previous_stats().label_stats().await.unwrap();

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

            SectionType::DiscoveredRecordings => {
                let all_stats = listens.current_stats().recording_stats().await.unwrap();
                let current_entity = listens.current_stats().recording_stats().await.unwrap();
                let previous_entity = listens.previous_stats().recording_stats().await.unwrap();

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

            SectionType::DiscoveredReleases => {
                let all_stats = listens.current_stats().release_stats().await.unwrap();
                let current_entity = listens.current_stats().release_stats().await.unwrap();
                let previous_entity = listens.previous_stats().release_stats().await.unwrap();

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

            SectionType::DiscoveredReleaseGroups => {
                let all_stats = listens.current_stats().release_group_stats().await.unwrap();
                let current_entity = listens.current_stats().release_group_stats().await.unwrap();
                let previous_entity = listens
                    .previous_stats()
                    .release_group_stats()
                    .await
                    .unwrap();

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

            SectionType::AllTimeChangesArtists => {
                let current_entity_all_time = listens
                    .current_all_time_stats()
                    .artists_stats()
                    .await
                    .unwrap();
                let previous_entity_all_time = listens
                    .previous_all_time_stats()
                    .artists_stats()
                    .await
                    .unwrap();
                all_time_changes(current_entity_all_time, previous_entity_all_time).await
            }

            SectionType::AllTimeChangesLabels => {
                let current_entity_all_time = listens
                    .current_all_time_stats()
                    .label_stats()
                    .await
                    .unwrap();
                let previous_entity_all_time = listens
                    .previous_all_time_stats()
                    .label_stats()
                    .await
                    .unwrap();
                all_time_changes(current_entity_all_time, previous_entity_all_time).await
            }

            SectionType::AllTimeChangesRecordings => {
                let current_entity_all_time = listens
                    .current_all_time_stats()
                    .recording_stats()
                    .await
                    .unwrap();
                let previous_entity_all_time = listens
                    .previous_all_time_stats()
                    .recording_stats()
                    .await
                    .unwrap();
                all_time_changes(current_entity_all_time, previous_entity_all_time).await
            }

            SectionType::AllTimeChangesReleases => {
                let current_entity_all_time = listens
                    .current_all_time_stats()
                    .release_stats()
                    .await
                    .unwrap();
                let previous_entity_all_time = listens
                    .previous_all_time_stats()
                    .release_stats()
                    .await
                    .unwrap();
                all_time_changes(current_entity_all_time, previous_entity_all_time).await
            }

            SectionType::AllTimeChangesReleaseGroups => {
                let current_entity_all_time = listens
                    .current_all_time_stats()
                    .release_group_stats()
                    .await
                    .unwrap();
                let previous_entity_all_time = listens
                    .previous_all_time_stats()
                    .release_group_stats()
                    .await
                    .unwrap();
                all_time_changes(current_entity_all_time, previous_entity_all_time).await
            }

            SectionType::DebutsRecordings => {
                let all_time_entity_stats =
                    listens.all_time_stats().recording_stats().await.unwrap();
                let current_entity_stats = listens
                    .current_all_time_stats()
                    .recording_stats()
                    .await
                    .unwrap();
                let previous_entity_stats = listens
                    .previous_all_time_stats()
                    .recording_stats()
                    .await
                    .unwrap();

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

            SectionType::DebutsReleases => {
                let all_time_entity_stats = listens.all_time_stats().release_stats().await.unwrap();
                let current_entity_stats = listens
                    .current_all_time_stats()
                    .release_stats()
                    .await
                    .unwrap();
                let previous_entity_stats = listens
                    .previous_all_time_stats()
                    .release_stats()
                    .await
                    .unwrap();

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
    }
}
