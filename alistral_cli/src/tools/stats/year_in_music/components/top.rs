use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::label::LabelWithReleases;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use itertools::Itertools as _;
use sequelles::datastructures::ranking::Ranking;

use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn top_labels(stats: Vec<LabelWithReleases>) -> String {
        let rankings = Ranking::from(stats);
        let rankings =
            rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let rows = rankings
            .into_iter()
            .map(|(rank, rec)| TopRow {
                ranking: rank + 1,
                score: TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default()),
                element: Box::new(rec.entity().clone()),
                previous_ranking: None,
                previous_score: None,
            })
            .collect_vec();

        TopPrinter::format_n_rows(rows, 20).await
    }
}
