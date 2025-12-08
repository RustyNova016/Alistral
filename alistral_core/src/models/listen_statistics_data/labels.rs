use snafu::ResultExt;
use snafu::Snafu;

use crate::datastructures::entity_with_listens::label::collection::LabelWithReleasesCollection;
use crate::datastructures::entity_with_listens::label::collection::sort::LabelStatsLinkingError;
use crate::models::listen_statistics_data::ListenStatisticsData;
use crate::models::listen_statistics_data::release::ReleaseStatsError;
use crate::traits::sorter::InsertElement;

impl ListenStatisticsData {
    /// Generate the label statistics based on the stored listens
    pub async fn label_stats(&self) -> Result<&LabelWithReleasesCollection, LabelStatsError> {
        self.labels.get_or_try_init(self.init_labels()).await
    }

    /// Create the label statistics
    async fn init_labels(&self) -> Result<LabelWithReleasesCollection, LabelStatsError> {
        let mut coll = LabelWithReleasesCollection::new();
        let recording_stats = self
            .release_stats()
            .await
            .context(ReleaseStatsSnafu)?
            .clone();

        coll.insert_element(self.client.as_ref(), recording_stats)
            .await
            .context(ReleaseGroupLinkingSnafu)?;

        Ok(coll)
    }
}

#[derive(Debug, Snafu)]
pub enum LabelStatsError {
    #[snafu(display("Couldn't generate release group statistics"))]
    ReleaseGroupLinkingError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: LabelStatsLinkingError,
    },

    #[snafu(display("Couldn't generate release statistics"))]
    ReleaseStatsError {
        #[cfg_attr(feature = "backtrace", snafu(backtrace))]
        source: ReleaseStatsError,
    },
}
