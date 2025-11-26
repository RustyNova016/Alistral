use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use itertools::Itertools;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    // pub async fn tags_report(&self) -> String {
    //     let mut out = String::new();
    //     let stats = self.current.label_stats().await.unwrap();
    //     let stats = stats.iter().cloned().collect_vec();

    //     writeln!(out, "{}", Heading1("Best tags of the year 🏆")).unwrap();
    //     writeln!(out, "{}", self.get_labels_distinct().await).unwrap();
    //     writeln!(out).unwrap();
    //     writeln!(out, "Here's the top 20 tags of this year:").unwrap();
    //     writeln!(out, "{}", Self::top_tags(stats).await).unwrap();

    //     out
    // }

    // async fn get_labels_distinct(&self) -> String {
    //     let current = self.current.label_stats().await.unwrap();
    //     let previous = self.previous.label_stats().await.unwrap();

    //     let current = current.iter_entities().count();
    //     let previous = previous.iter_entities().count();

    //     format!(
    //         "You listened to {} distinct labels this year ({} {})",
    //         current.alistral_green(),
    //         ComparisonArrow::greater_is_better(current, previous),
    //         previous.alistral_green(),
    //     )
    // }


}
