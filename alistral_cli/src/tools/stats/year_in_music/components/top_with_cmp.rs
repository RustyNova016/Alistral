use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::label::collection::LabelWithReleasesCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use duplicate::duplicate_item;
use itertools::Itertools as _;
use sequelles::datastructures::ranking::Ranking;

use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::models::datastructures::tops::printer::top_cell::TopCell;
use crate::models::datastructures::tops::printer::top_columns::TopColumnSort;
use crate::models::datastructures::tops::printer::top_columns::TopColumnType;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::printer::top_table_printer::TopTablePrinter;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    #[duplicate_item(
        method       stat_type;
        [top_recordings_with_cmp]       [RecordingWithListensCollection];
        [top_artists_with_cmp]          [ArtistWithRecordingsCollection];
        [top_release_groups_with_cmp]   [ReleaseGroupWithReleasesCollection];
        [top_labels_with_cmp]           [LabelWithReleasesCollection];
    )]
    pub async fn method(&self, mut stats: stat_type, mut prev: stat_type) -> String {
        // Get all the entities
        let entities = stats
            .iter()
            .chain(prev.iter())
            .map(|rec| rec.entity())
            .unique_by(|rec| &rec.mbid)
            .cloned()
            .collect_vec();

        // Make sure both collection have all the entities
        for entity in entities {
            stats.insert_or_merge_entity(entity.clone());
            prev.insert_or_merge_entity(entity);
        }

        let rankings = Ranking::from(stats.into_iter().collect_vec());
        let rankings =
            rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let prev_rankings = Ranking::from(prev.into_iter().collect_vec());
        let prev_rankings =
            prev_rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let rows = rankings
            .into_iter()
            .map(|(rank, rec)| {
                let prev = prev_rankings
                    .iter()
                    .find(|prev| prev.1.entity().mbid == rec.entity().mbid);

                let previous_score = prev
                    .as_ref()
                    .map(|(_, rec)| rec.get_time_listened().unwrap_or_default());

                TopRow {
                    element: Box::new(rec.entity().clone()),

                    ranking: Some(TopCell::new(
                        Some(rank + 1),
                        prev.as_ref().map(|(rank, _)| rank + 1),
                        true,
                    )),

                    listen_duration: Some(TopCell::new(
                        Some(HumanTimePrinter::from(rec.get_time_listened())),
                        Some(HumanTimePrinter::from(previous_score)),
                        true,
                    )),

                    listen_count: Some(TopCell::new(
                        Some(rec.listen_count()),
                        prev.as_ref().map(|prev| prev.1.listen_count()),
                        true,
                    )),
                }
            })
            .collect_vec();

        let table = TopTablePrinter::builder()
            .columns(self.get_top_columns())
            .sorted_column(TopColumnType::ListenDuration)
            .sort_order(TopColumnSort::Desc)
            .build();

        table.format_n_rows(rows, 20).await
    }

    pub fn get_top_columns(&self) -> Vec<TopColumnType> {
        let mut cols = vec![TopColumnType::Rank, TopColumnType::ListenDuration];

        if self.listen_counts {
            cols.push(TopColumnType::ListenCount);
        }

        cols.push(TopColumnType::Title);

        cols
    }
}
