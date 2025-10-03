use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::models::listen_statistics_data::ListenStatisticsData;

impl ListenStatisticsData {
    /// Generate the recording statistics based on the stored listens
    pub async fn recording_stats(&self) -> Result<&RecordingWithListensCollection, crate::Error> {
        self.recordings.get_or_try_init(self.init_recording()).await
    }

    /// Create the recording statistics
    async fn init_recording(&self) -> Result<RecordingWithListensCollection, crate::Error> {
        RecordingWithListensCollection::from_listencollection(
            &self.client,
            self.listens.clone(),
            &self.client.recording_with_listen_strat,
        )
        .await
    }
}
