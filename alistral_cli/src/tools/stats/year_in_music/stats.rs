use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn prefetch_data(&self) {
        self.full_user_stats.recording_stats().await.unwrap();
        self.full_user_stats.artists_stats().await.unwrap();
        self.full_user_stats.release_group_stats().await.unwrap();

        self.current.recording_stats().await.unwrap();
        self.current.artists_stats().await.unwrap();
        self.current.release_group_stats().await.unwrap();
    }

    pub async fn num_listens_in_year(&self) -> usize {
        self.current.listens().listen_count()
    }

    pub async fn get_month_rec_stats(
        &self,
        year: i32,
        month: u32,
    ) -> (
        RecordingWithListensCollection,
        RecordingWithListensCollection,
    ) {
        (
            self.full_user_stats
                .clone_no_stats()
                .filter_on_year_month(year, month)
                .recording_stats()
                .await
                .unwrap()
                .to_owned(),
            if month == 1 {
                self.full_user_stats
                    .clone_no_stats()
                    .filter_on_year_month(year - 1, 12)
                    .recording_stats()
                    .await
                    .unwrap()
                    .to_owned()
            } else {
                self.full_user_stats
                    .clone_no_stats()
                    .filter_on_year_month(year, month - 1)
                    .recording_stats()
                    .await
                    .unwrap()
                    .to_owned()
            },
        )
    }
}
