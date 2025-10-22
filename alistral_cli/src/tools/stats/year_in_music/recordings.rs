use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::datastructures::mappings::listen_release_year::ListenByReleaseDate;
use chrono::Datelike;
use itertools::Itertools;
use sequelles::datastructures::ranking::Ranking;

use crate::ALISTRAL_CLIENT;
use crate::models::datastructures::tops::scorer::listen_count;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn recording_report(&self) -> String {
        self.new_releases().await
    }

    async fn new_releases(&self) -> String {
        let recordings = self.current.recording_stats().await.unwrap();

        let mut listen_by_dates = ListenByReleaseDate::default();
        listen_by_dates
            .insert_recording_stats_collection(&ALISTRAL_CLIENT.core, recordings.to_owned()).await.unwrap();

        listen_by_dates
            .mapping
            .as_mut_hash_map()
            .retain(|_, (date, _)| date.is_some_and(|date| date.year() == self.year));

        let track_count = listen_by_dates.mapping.as_hash_map().values().flat_map(|row| &row.1).count();
        let listen_count = listen_by_dates.mapping.as_hash_map().values().flat_map(|row| &row.1).flat_map(|rec| rec.iter_listens()).count();

        format!("You listened to {} tracks released this year,\nResulting in {} listens", track_count, listen_count)
    }

    async fn new_releases_top(&self, listen_by_dates: &ListenByReleaseDate) {
        let rankings = Ranking::from(listen_by_dates.mapping.as_hash_map().values().collect_vec());
        let rankings = rankings.get_ranks(|(date)| date);

        let rows = rankings.into_iter().map(|(rank, (date, recs))| {
            
        });
    }
}
