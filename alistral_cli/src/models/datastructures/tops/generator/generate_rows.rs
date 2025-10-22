use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Recording;
use sequelles::datastructures::ranking::Ranking;

use crate::models::datastructures::tops::generator::TopGenerator;
use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_entity::TopEntity;

impl TopGenerator {
    pub async fn generate_recording_rows<Sco>(&self, scorer: Sco) -> Vec<TopRow>
    where
        Sco: TopScorer<RecordingWithListens> + 'static,
    {
        let (cur_stats, previous_stats, recordings) = self.get_recording_stats().await;

        let cur_stats = get_ranks_for_stats(&recordings, cur_stats, &scorer);
        let previous_stats =
            previous_stats.map(|stats| get_ranks_for_stats(&recordings, stats, &scorer));

        Self::generate_top_rows(cur_stats, previous_stats, &scorer)
    }

    pub async fn print_recording_stats<Sco>(&self, scorer: Sco)
    where
        Sco: TopScorer<RecordingWithListens> + 'static,
    {
        let stats = self.generate_recording_rows(scorer).await;
        TopPrinter::print_rows(stats).await;
    }

    /// Generate the rows from the rankings
    pub fn generate_top_rows<Ent, Sco>(
        cur_stats: Vec<(usize, Ent)>,
        before_stats: Option<Vec<(usize, Ent)>>,
        scorer: &Sco,
    ) -> Vec<TopRow>
    where
        Ent: TopEntity<Sco> + 'static,
        Sco: TopScorer<Ent> + 'static,
    {
        let mut rows = Vec::new();
        for (rank, recording) in cur_stats {
            let before = before_stats.as_ref().and_then(|before| {
                before
                    .iter()
                    .find(|(_, rec)| rec.rowid() == recording.rowid())
            });

            rows.push(TopRow {
                ranking: rank + 1,
                score: scorer.get_score_of_element(&recording),
                element: Box::new(recording.get_display_entity()),

                previous_ranking: before.as_ref().map(|(rank, _)| rank + 1),
                previous_score: before.map(|previous| scorer.get_score_of_element(&previous.1)),
            })
        }

        rows
    }
}

fn get_ranks_for_stats<Sco>(
    ents: &Vec<Recording>,
    stats: RecordingWithListensCollection,
    scorer: &Sco,
) -> Vec<(usize, RecordingWithListens)>
where
    Sco: TopScorer<RecordingWithListens>,
{
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

    ranking.get_ranks(|rec| Reverse(scorer.get_score_of_element(&rec)))
}
