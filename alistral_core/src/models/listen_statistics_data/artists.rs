use snafu::ResultExt;
use snafu::Snafu;

use crate::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use crate::datastructures::entity_with_listens::artist::collection::sort::ArtistStatsLinkingError;
use crate::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use crate::models::listen_statistics_data::ListenStatisticsData;
use crate::traits::sorter::InsertElement;

impl ListenStatisticsData {
    /// Generate the recording statistics based on the stored listens
    pub async fn artists_stats(&self) -> Result<&ArtistWithRecordingsCollection, ArtistStatsError> {
        self.artists.get_or_try_init(self.init_artists()).await
    }

    /// Create the recording statistics
    async fn init_artists(&self) -> Result<ArtistWithRecordingsCollection, ArtistStatsError> {
        let mut coll = ArtistWithRecordingsCollection::new();
        let recording_stats = self
            .recording_stats()
            .await
            .context(RecordingStatsSnafu)?
            .clone();

        coll.insert_element(self.client.as_ref(), recording_stats)
            .await
            .context(ArtistLinkingSnafu)?;
        Ok(coll)
    }
}

#[derive(Debug, Snafu)]
pub enum ArtistStatsError {
    #[snafu(display("Couldn't generate artist statistics"))]
    ArtistLinkingError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: ArtistStatsLinkingError,
    },

    #[snafu(display("Couldn't generate recording statistics"))]
    RecordingStatsError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: RecordingStatsError,
    },
}
