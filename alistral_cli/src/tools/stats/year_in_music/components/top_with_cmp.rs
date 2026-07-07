use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use alistral_core::datastructures::entity_with_listens::label::collection::LabelWithReleasesCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
use duplicate::duplicate_item;
use itertools::Itertools as _;

use crate::models::cli_components::tables::order_by::OrderTableByListenDuration;
use crate::models::cli_components::tables::rows::top_listen_dur_count::TopListenDurCountRow;
use crate::models::cli_components::tables::rows::top_listen_duration_row::TopListenDurationRow;
use crate::models::cli_components::tables::table::TopTable;
use crate::tools::stats::year_in_music::YimReport;
use crate::utils::extensions::sequelles::hashjoin::HashJoin as _;

impl YimReport {
    #[duplicate_item(
        method       stat_type;
        [top_recordings_with_cmp]       [RecordingWithListensCollection];
        [top_artists_with_cmp]          [ArtistWithRecordingsCollection];
        [top_release_groups_with_cmp]   [ReleaseGroupWithReleasesCollection];
        [top_labels_with_cmp]           [LabelWithReleasesCollection];
    )]
    pub async fn method(&self, stats: stat_type, prev: stat_type) -> String {
        let join = stats.0.hash_join(&prev.0);
        let stats = join
            .into_values()
            .map(|(current, previous)| {
                EntityListensComparison::new(current.cloned(), previous.cloned())
            })
            .collect_vec();

        if self.listen_counts {
            let table: TopTable<TopListenDurCountRow<_>, OrderTableByListenDuration> =
                TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
            table.format(20, 0).await
        } else {
            let table: TopTable<TopListenDurationRow<_>, OrderTableByListenDuration> =
                TopTable::from_entity_listens_comps(stats, OrderTableByListenDuration, true);
            table.format(20, 0).await
        }
    }
}
