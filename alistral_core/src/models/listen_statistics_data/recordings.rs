use itertools::Itertools;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use crate::models::listen_statistics_data::ListenStatisticsData;
use crate::traits::sorter::InsertElement;

impl ListenStatisticsData {
    /// Generate the recording statistics based on the stored listens
    pub async fn recording_stats(
        &self,
    ) -> Result<&RecordingWithListensCollection, RecordingStatsError> {
        self.recordings.get_or_try_init(self.init_recording()).await
    }

    /// Create the recording statistics
    async fn init_recording(&self) -> Result<RecordingWithListensCollection, RecordingStatsError> {
        let mut coll = RecordingWithListensCollection::new();
        coll.insert_elements(
            self.client.as_ref(),
            self.listens.iter().cloned().collect_vec(),
        )
        .await?;
        Ok(coll)
    }
}
