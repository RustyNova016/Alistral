use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn prefetch_data(&self) {
        pg_counted!(6, "Fetching data");

        self.full_user_stats.recording_stats().await.unwrap();
        pg_inc!();
        self.full_user_stats.artists_stats().await.unwrap();
        pg_inc!();
        self.full_user_stats.release_stats().await.unwrap();
        pg_inc!();
        self.full_user_stats.release_group_stats().await.unwrap();
        pg_inc!();
        self.full_user_stats.label_stats().await.unwrap();
        pg_inc!();

        self.current.recording_stats().await.unwrap();
        self.current.artists_stats().await.unwrap();
        self.current.release_group_stats().await.unwrap();
        pg_inc!();
    }

    pub async fn num_listens_in_year(&self) -> usize {
        self.current.listens().listen_count()
    }
}
