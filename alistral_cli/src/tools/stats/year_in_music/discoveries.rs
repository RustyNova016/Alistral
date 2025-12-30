use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn discoveries_page(&self) -> String {
        let mut out = String::new();
        writeln!(out, "{}", Heading1("Discoveries of the year 🔎")).unwrap();

        let stats = self.data.discovery_years_current().await;

        if !stats.is_empty() {
            writeln!(out, "{}", self.new_discoveries().await).unwrap();
            writeln!(out).unwrap();
            writeln!(out, "Here's the top 20 tracks:").unwrap();
            writeln!(out, "{}", self.top_recordings(stats.to_owned()).await).unwrap();
        } else {
            writeln!(out, "{}", self.no_new_discoveries().await).unwrap();
        }

        out
    }

    async fn new_discoveries(&self) -> String {
        let current_year = self.data.discovery_years_current().await;

        let track_count = current_year.len();
        let listen_count: usize = current_year.iter().map(|rec| rec.listen_count()).sum();
        let listen_perc = Decimal::new(listen_count as i64, 0)
            / Decimal::new(self.num_listens_in_year().await as i64, 0);

        let previous = self.data.discovery_years_previous().await;
        if !previous.is_empty() {
            let track_count_prev = previous.len();
            let listen_count_prev: usize = previous.iter().map(|rec| rec.listen_count()).sum();

            format!(
                "You discovered {} tracks this year [{} {}], which make up a total of {} listens [{} {}] ({}% of this year's listens)",
                track_count.alistral_green(),
                ComparisonArrow::greater_is_better(track_count, track_count_prev),
                track_count_prev.alistral_green(),
                listen_count.alistral_green(),
                ComparisonArrow::greater_is_better(listen_count, listen_count_prev),
                listen_count_prev.alistral_green(),
                listen_perc
                    .checked_mul(dec!(100))
                    .unwrap()
                    .trunc_with_scale(2)
                    .alistral_green()
            )
        } else {
            format!(
                "You discovered {} tracks this year, which make up a total of {} listens ({}% of this year's listens)",
                track_count.alistral_green(),
                listen_count.alistral_green(),
                listen_perc
                    .checked_mul(dec!(100))
                    .unwrap()
                    .trunc_with_scale(2)
                    .alistral_green()
            )
        }
    }

    async fn no_new_discoveries(&self) -> String {
        let mut out = "You haven't discovered any tracks this year...".to_string();

        let previous = self.data.discovery_years_previous().await;
        if !previous.is_empty() {
            let track_count = previous.len();
            let listen_count: usize = previous.iter().map(|rec| rec.listen_count()).sum();
            writeln!(
                out,
                "Which is less than last year's {track_count} discoveries ({listen_count} listens)"
            )
            .unwrap();
        }

        out
    }
}
