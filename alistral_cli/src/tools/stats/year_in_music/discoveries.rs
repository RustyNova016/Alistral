use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors as _;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Datelike;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::cli_formating::title::Heading1;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn discoveries_page(&self) -> String {
        let mut out = String::new();
        let stats = self.get_discoveries_stats().await;
        writeln!(out, "{}", Heading1("Discoveries of the year 🔎")).unwrap();

        writeln!(out, "{}", self.discoveries(&stats).await).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Here's the top 20 tracks:").unwrap();
        writeln!(out, "{}", Self::top_recordings(stats).await).unwrap();

        out
    }

    async fn get_discoveries_stats(&self) -> Vec<RecordingWithListens> {
        let recordings = self.full_user_stats.recording_stats().await.unwrap();

        let discoveries = recordings
            .iter()
            .filter(|rec| {
                rec.listens()
                    .oldest_listen_date()
                    .map(|date| date.year())
                    .unwrap_or_default()
                    == self.year
            })
            .map(|rec| rec.entity().rowid())
            .collect_vec();

        self.current
            .recording_stats()
            .await
            .unwrap()
            .iter()
            .filter(|rec| discoveries.contains(&rec.rowid()))
            .cloned()
            .collect_vec()
    }

    async fn discoveries(&self, stats: &[RecordingWithListens]) -> String {
        let track_count = stats.len();
        let listen_count: usize = stats.iter().map(|rec| rec.listen_count()).sum();
        let listen_perc = Decimal::new(listen_count as i64, 0)
            / Decimal::new(self.num_listens_in_year().await as i64, 0);

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
