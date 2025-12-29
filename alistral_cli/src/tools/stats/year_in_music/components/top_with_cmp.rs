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

use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    #[duplicate_item(
        method       stat_type;
        [top_recordings_with_cmp]       [RecordingWithListensCollection];
        [top_artists_with_cmp]          [ArtistWithRecordingsCollection];
        [top_release_groups_with_cmp]   [ReleaseGroupWithReleasesCollection];
        [top_labels_with_cmp]           [LabelWithReleasesCollection];
    )]
    pub async fn method(mut stats: stat_type, mut prev: stat_type) -> String {
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

                let previous_score = prev.as_ref().map(|(_, rec)| {
                    TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default())
                });

                TopRow {
                    ranking: rank + 1,
                    listen_count: rec.listen_count(),
                    score: TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default()),
                    element: Box::new(rec.entity().clone()),
                    previous_ranking: prev.as_ref().map(|(rank, _)| rank + 1),
                    previous_score,
                }
            })
            .collect_vec();

        TopPrinter::format_n_rows(rows, 20).await
    }
}
