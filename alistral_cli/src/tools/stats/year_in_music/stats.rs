use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;

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
}
