use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Recording;
use sequelles::datastructures::ranking::Ranking;

use crate::interface::tops::top_row::TopRow;
use crate::models::datastructures::tops::display_top::DisplayTop;
use crate::models::datastructures::tops::scorer::TopScorer;
use crate::tools::stats::tops::generator::TopGenerator;

impl TopGenerator {
    pub async fn generate_recording_rows<Sco>(&self, scorer: Sco) -> Vec<TopRow<Recording>>
    where
        Sco: TopScorer<RecordingWithListens> + 'static,
    {
        let (cur_stats, previous_stats, recordings) = self.get_recording_stats().await;

        let cur_stats = get_ranks_for_stats(&recordings, cur_stats);
        let previous_stats = previous_stats.map(|stats| get_ranks_for_stats(&recordings, stats));

        generate_top_rows(cur_stats, previous_stats, scorer)
    }

    pub async fn print_recording_stats<Sco>(&self, scorer: Sco)
    where
        Sco: TopScorer<RecordingWithListens> + 'static,
    {
        let stats = self.generate_recording_rows(scorer).await;
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
            None => ranking.push(RecordingWithListens::new(
                Recording::clone(ent),
                Default::default(),
            )),
        }
    }

    ranking.get_ranks(|rec| Reverse(rec.listen_count()))
}

fn generate_top_rows<Ent, Lis, Sco>(
    cur_stats: Vec<(usize, EntityWithListens<Ent, Lis>)>,
    before_stats: Option<Vec<(usize, EntityWithListens<Ent, Lis>)>>,
    scorer: Sco,
) -> Vec<TopRow<Ent>>
where
    Ent: HasRowID + Clone + 'static,
    Lis: ListenCollectionReadable + 'static,
    EntityWithListens<Ent, Lis>: Clone,
    Sco: TopScorer<EntityWithListens<Ent, Lis>>,
{
    let mut rows = Vec::new();
    for (rank, recording) in cur_stats {
        let before = before_stats.as_ref().and_then(|before| {
            before
                .iter()
                .find(|(_, rec)| rec.entity().rowid() == recording.entity().rowid())
        });

        let previous_score = match before {
            Some(prev_ent) => {
                Some(scorer.box_score(scorer.get_score_of_element(prev_ent.1.to_owned())))
            }
            None => None,
        };

        rows.push(TopRow {
            ranking: rank + 1,
            score: Box::new(scorer.get_score_of_element(recording.clone())),
            element: recording.into_entity(),

            previous_ranking: before.as_ref().map(|(rank, _)| rank + 1),
            previous_score: previous_score,
        })
    }

    rows
}
