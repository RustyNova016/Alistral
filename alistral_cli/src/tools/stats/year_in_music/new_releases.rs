use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors as _;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::cli_formating::title::Heading1;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn new_release_page(&self) -> String {
        let mut out = String::new();
        writeln!(out, "{}", Heading1("New releases of the year 🌱")).unwrap();

        if let Some(stats) = self
            .data
            .release_years_current()
            .await
            .get(&(self.year as u64))
        {
            writeln!(out, "{}", self.new_releases().await).unwrap();
            writeln!(out).unwrap();
            writeln!(out, "Here's the top 20 tracks:").unwrap();
            writeln!(out, "{}", self.top_recordings(stats.to_owned()).await).unwrap();
        } else {
            writeln!(out, "{}", self.no_new_releases().await).unwrap();
        }

        out
    }

    async fn new_releases(&self) -> String {
        let current_year = self
            .data
            .release_years_current()
            .await
            .get(&(self.year as u64))
            .expect("no_new_releases should have been called instead");

        let track_count = current_year.len();
        let listen_count: usize = current_year.iter().map(|rec| rec.listen_count()).sum();
        let listen_perc = Decimal::new(listen_count as i64, 0)
            / Decimal::new(self.num_listens_in_year().await as i64, 0);

        if let Some(previous) = self
            .data
            .release_years_previous()
            .await
            .get(&((self.year - 1) as u64))
        {
            let track_count_prev = previous.len();
            let listen_count_prev: usize = previous.iter().map(|rec| rec.listen_count()).sum();

            format!(
                "You listened to {} tracks released this year [{} {}], which make up a total of {} listens [{} {}] ({}% of this year's listens)",
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
                "You listened to {} tracks released this year, which make up a total of {} listens ({}% of this year's listens)",
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

    async fn no_new_releases(&self) -> String {
        let mut out = "You haven't listened to any tracks released this year...".to_string();

        if let Some(previous) = self
            .data
            .release_years_previous()
            .await
            .get(&((self.year - 1) as u64))
        {
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
