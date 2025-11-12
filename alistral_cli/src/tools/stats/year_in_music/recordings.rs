use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use itertools::Itertools;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn recording_report(&self) -> String {
        let mut out = String::new();
        let stats = self.current.recording_stats().await.unwrap();
        let stats = stats.iter().cloned().collect_vec();

        writeln!(out, "{}", Heading1("Best recordings of the year 🏆")).unwrap();
        writeln!(out, "{}", self.get_recording_distinct().await).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Here's the top 20 tracks of this year:").unwrap();
        writeln!(out, "{}", Self::top_recordings(stats).await).unwrap();

        out
    }

    async fn get_recording_distinct(&self) -> String {
        let current = self.current.recording_stats().await.unwrap();
        let previous = self.previous.recording_stats().await.unwrap();

        let current = current.iter_entities().count();
        let previous = previous.iter_entities().count();

        format!(
            "You listened to {} distinct recordings this year ({} {})",
            current.alistral_green(),
            ComparisonArrow::greater_is_better(current, previous),
            previous.alistral_green(),
        )
    }

    // async fn new_releases_top(&self, listen_by_dates: &ListenByReleaseDate) {
    //     let rankings = Ranking::from(listen_by_dates.mapping.as_hash_map().values().collect_vec());
    //     let rankings = rankings.get_ranks(|(date)| date);

    //     let rows = rankings.into_iter().map(|(rank, (date, recs))| {

    //     });
    // }
}
