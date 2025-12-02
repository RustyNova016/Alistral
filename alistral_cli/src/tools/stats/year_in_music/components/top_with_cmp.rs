use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use itertools::Itertools as _;
use sequelles::datastructures::ranking::Ranking;

use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn top_recordings_with_cmp(
        stats: Vec<RecordingWithListens>,
        prev: Vec<RecordingWithListens>,
    ) -> String {
        let rankings = Ranking::from(stats);
        let rankings =
            rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let prev_rankings = Ranking::from(prev);
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
                    score: TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default()),
                    element: Box::new(rec.recording().clone()),
                    previous_ranking: prev.as_ref().map(|(rank, _)| rank + 1),
                    previous_score,
                }
            })
            .collect_vec();

        TopPrinter::format_n_rows(rows, 20).await
    }
}
