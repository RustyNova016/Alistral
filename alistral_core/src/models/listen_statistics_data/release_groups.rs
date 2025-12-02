use snafu::ResultExt;
use snafu::Snafu;

use crate::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
use crate::datastructures::entity_with_listens::release_group::collection::sort::ReleaseGroupStatsLinkingError;
use crate::models::listen_statistics_data::ListenStatisticsData;
use crate::models::listen_statistics_data::release::ReleaseStatsError;
use crate::traits::sorter::InsertElement;

impl ListenStatisticsData {
    /// Generate the recording statistics based on the stored listens
    pub async fn release_group_stats(
        &self,
    ) -> Result<&ReleaseGroupWithReleasesCollection, ReleaseGroupStatsError> {
        self.release_groups
            .get_or_try_init(self.init_release_groups())
            .await
    }

    /// Create the recording statistics
    async fn init_release_groups(
        &self,
    ) -> Result<ReleaseGroupWithReleasesCollection, ReleaseGroupStatsError> {
        let mut coll = ReleaseGroupWithReleasesCollection::new();
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
pub enum ReleaseGroupStatsError {
    #[snafu(display("Couldn't generate release group statistics"))]
    ReleaseGroupLinkingError {
        #[snafu(backtrace)]
        source: ReleaseGroupStatsLinkingError,
    },

    #[snafu(display("Couldn't generate release statistics"))]
    ReleaseStatsError {
        #[snafu(backtrace)]
        source: ReleaseStatsError,
    },
}
