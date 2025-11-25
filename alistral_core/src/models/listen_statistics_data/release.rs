use snafu::ResultExt;
use snafu::Snafu;

use crate::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::datastructures::entity_with_listens::release::collection::sort::ReleaseStatsLinkingError;
use crate::models::listen_statistics_data::ListenStatisticsData;
use crate::traits::sorter::InsertElement;

impl ListenStatisticsData {
    /// Generate the recording statistics based on the stored listens
    pub async fn release_stats(
        &self,
    ) -> Result<&ReleaseWithRecordingsCollection, ReleaseStatsError> {
        self.releases.get_or_try_init(self.init_releases()).await
    }

    /// Create the recording statistics
    async fn init_releases(&self) -> Result<ReleaseWithRecordingsCollection, ReleaseStatsError> {
        let mut coll = ReleaseWithRecordingsCollection::new();
        let recording_stats = self
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?
            .clone();

        coll.insert_element(self.client.as_ref(), recording_stats)
            .await
            .context(ReleaseLinkingSnafu)?;

        Ok(coll)
    }
}

#[derive(Debug, Snafu)]
pub enum ReleaseStatsError {
    #[snafu(display("Couldn't generate artist statistics"))]
    ReleaseLinkingError {
        #[snafu(backtrace)]
        source: ReleaseStatsLinkingError,
    },

    #[snafu(display("Couldn't generate recording statistics"))]
    RecordingStatsError {
        #[snafu(backtrace)]
        source: RecordingStatsError,
    },
}
