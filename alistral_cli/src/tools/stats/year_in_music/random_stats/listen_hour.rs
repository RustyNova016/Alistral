use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub(super) async fn best_listen_hour(&self) -> Option<String> {
        let current_top = self.data.listen_per_hourmin_current().await;
        let current_top = current_top.iter().max_by_key(|(_, count)| count.len())?;

        let previous_top = self.data.listen_per_hourmin_previous().await;
        let previous_top = previous_top.iter().max_by_key(|(_, count)| count.len());

        let mut out = format!(
            "Listened the most tracks at {} ({} listens).",
            current_top.0.alistral_green(),
            current_top.1.len().alistral_green(),
        );

        if let Some(previous_top) = previous_top {
            write!(
                out,
                " Last year was {} ({} {} listens).",
                previous_top.0.alistral_green(),
                ComparisonArrow::greater_is_better(current_top.1.len(), previous_top.1.len()),
                previous_top.1.len().alistral_green()
            )
            .unwrap()
        }

        Some(out)
    }
}
