use core::cmp::Reverse;
use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::release_group::ReleaseGroupWithReleases;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime as _;
use itertools::Itertools;
use sequelles::datastructures::ranking::Ranking;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn release_report(&self) -> String {
        let mut out = String::new();
        let stats = self.current.release_group_stats().await.unwrap();
        let stats = stats.iter().cloned().collect_vec();

        writeln!(out, "{}", Heading1("Best releases of the year 🏆")).unwrap();
        writeln!(out, "{}", self.get_releases_distinct().await).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Here's the top 20 releases of this year:").unwrap();
        writeln!(out, "{}", Self::top_releases(stats).await).unwrap();

        out
    }

    async fn get_releases_distinct(&self) -> String {
        let current = self.current.release_group_stats().await.unwrap();
        let previous = self.previous.release_group_stats().await.unwrap();

        let current = current.iter_entities().count();
        let previous = previous.iter_entities().count();

        format!(
            "You listened to {} distinct releases this year ({} {})",
            current.alistral_green(),
            ComparisonArrow::greater_is_better(current, previous),
            previous.alistral_green(),
        )
    }

    pub async fn top_releases(stats: Vec<ReleaseGroupWithReleases>) -> String {
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
