use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::cli_formating::title::Heading1;
use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub(super) async fn random_stats_report(&self) -> String {
        let mut out = String::new();
        writeln!(out, "{}", Heading1("General stats 📈")).unwrap();
        writeln!(out, "In {}, you:", self.year).unwrap();
        writeln!(out, " - {}", self.listen_count_line()).unwrap();
        writeln!(out, " - {}", self.listen_duration_line().await).unwrap();
        out
    }

    fn listen_count_line(&self) -> String {
        let current = self.current.listens().listen_count();
        let previous = self.previous.listens().listen_count();
        format!(
            "Listened to {} tracks ({} {})",
            current.alistral_green(),
            ComparisonArrow::greater_is_better(current, previous),
            previous.alistral_green()
        )
    }

    async fn listen_duration_line(&self) -> String {
        let current = self
            .current
            .recording_stats()
            .await
            .unwrap()
            .get_time_listened();

        let previous = self
            .previous
            .recording_stats()
            .await
            .unwrap()
            .get_time_listened();

        let secs_in_year = (self.year_end - self.year_start).num_seconds();
        let sec_listened = current.clone().map(|dur| dur.num_seconds()).unwrap_or(0);
        let year_percent = (Decimal::new(sec_listened, 0) / Decimal::new(secs_in_year, 0)) * dec!(100);

        format!(
            "Had music in your ears for {} ({} {}). That's {}% of the year!",
            HumanTimePrinter::from(current).alistral_green(),
            ComparisonArrow::greater_is_better(current, previous),
            HumanTimePrinter::from(previous).alistral_green(),
            year_percent.round_dp(2).alistral_green()
        )
    }
}
