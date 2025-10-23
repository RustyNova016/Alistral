use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Recording;
use sequelles::datastructures::ranking::Ranking;

use crate::interface::tops::top_row::TopRow;
use crate::tools::stats::tops::generator::TopGenerator;

impl TopGenerator {
    pub async fn generate_recording_rows(&self) -> Vec<TopRow<Recording>> {
        let (cur_stats, previous_stats, recordings) = self.get_recording_stats().await;

        let cur_stats =  get_ranks_for_stats(&recordings, cur_stats);
        let previous_stats = previous_stats.map(|stats| get_ranks_for_stats(&recordings, stats));

        generate_top_rows(cur_stats, previous_stats)
    }

    pub async fn print_recording_stats(&self) {
        let stats = self.generate_recording_rows().await;
        Self::print_rows(stats).await;
    }
}

fn get_ranks_for_stats(
    ents: &Vec<Recording>,
    stats: RecordingWithListensCollection,
) -> Vec<(usize, RecordingWithListens)> {
    let mut ranking = Ranking::new();

    for ent in ents {
        match stats.get_by_id(ent.rowid()) {
            Some(stat) => ranking.push(stat.to_owned()),
            None => ranking.push(RecordingWithListens::new(Recording::clone(ent), Default::default())),
        }
    }

    ranking.get_ranks(|rec| Reverse(rec.listen_count()))
}

fn generate_top_rows<Ent, Lis>(
    cur_stats: Vec<(usize, EntityWithListens<Ent, Lis>)>,
    before_stats: Option<Vec<(usize, EntityWithListens<Ent, Lis>)>>,
) -> Vec<TopRow<Ent>>
where
    Ent: HasRowID + Clone,
    Lis: ListenCollectionReadable,
{
    let mut rows = Vec::new();
    for (rank, recording) in cur_stats {
        let before = before_stats.as_ref().and_then(|before| {
            before
                .iter()
                .find(|(_, rec)| rec.entity().rowid() == recording.entity().rowid())
        });

        rows.push(TopRow {
            ranking: rank + 1,
            score: recording.listen_count().to_string(),
            element: recording.into_entity(),

            previous_ranking: before.as_ref().map(|(rank, _)| rank + 1),
            previous_score: before.map(|(_, rec)| rec.listen_count().to_string()),
        })
    }

    rows
}
