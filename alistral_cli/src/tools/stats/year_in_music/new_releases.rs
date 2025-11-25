use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors as _;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::cli_formating::title::Heading1;
use crate::datastructures::mappers::stats_by_release_year::StatsByReleaseYear;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn new_release_page(&self) -> String {
        let mut out = String::new();
        let stats = self.get_stats().await;
        writeln!(out, "{}", Heading1("New releases of the year 🌱")).unwrap();

        writeln!(out, "{}", self.new_releases(&stats).await).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Here's the top 20 tracks:").unwrap();
        writeln!(out, "{}", Self::top_recordings(stats).await).unwrap();

        out
    }

    async fn get_stats(&self) -> Vec<RecordingWithListens> {
        let recordings = self.current.recording_stats().await.unwrap();

        let mut listen_by_dates = StatsByReleaseYear::default();
        listen_by_dates
            .insert_recording_stats_collection(&ALISTRAL_CLIENT.core, recordings.to_owned())
            .await
            .unwrap();

        listen_by_dates
            .mapping
            .as_mut_hash_map()
            .retain(|_, (date, _)| date.is_some_and(|date| date == self.year as i64));

        listen_by_dates
            .mapping
            .into_iter()
            .flat_map(|(_, row)| row)
            .collect_vec()
    }

    async fn new_releases(&self, stats: &Vec<RecordingWithListens>) -> String {
        let track_count = stats.len();
        let listen_count: usize = stats.iter().map(|rec| rec.listen_count()).sum();
        let listen_perc = Decimal::new(listen_count as i64, 0)
            / Decimal::new(self.num_listens_in_year().await as i64, 0);

        format!(
            "You listened to {} tracks released this year, which make up a total of {} listens ({}% of this year's listens)",
            track_count.alistral_green(),
            listen_count.alistral_green(),
            listen_perc.checked_mul(dec!(100)).unwrap().trunc_with_scale(2).alistral_green()
        )
    }
}
