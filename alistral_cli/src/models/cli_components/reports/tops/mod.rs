use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use alistral_core::models::listen_statistics_data::artists::ArtistStatsError;
use alistral_core::models::listen_statistics_data::labels::LabelStatsError;
use alistral_core::models::listen_statistics_data::release::ReleaseStatsError;
use alistral_core::models::listen_statistics_data::release_groups::ReleaseGroupStatsError;
use duplicate::duplicate_item;
use snafu::ResultExt;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::cli_components::reports::tops::generic_report::generic_top_report;

pub mod generic_report;

#[duplicate_item(
        method                          get_stats              error_selector;
        [top_recordings_report]         [recording_stats]      [RecordingStatsSnafu];
        [top_artists_report]            [artists_stats]        [ArtistStatsSnafu];
        [top_releases_report]           [release_stats]        [ReleaseStatsSnafu];
        [top_releases_group_report]     [release_group_stats]  [ReleaseGroupStatsSnafu];
        [top_label_report]              [label_stats]          [LabelStatsSnafu];
    )]
pub async fn method(
    current_stats: &ListenStatisticsData,
    previous_stats: &ListenStatisticsData,
) -> Result<String, TopReportError> {
    let current_recordings = current_stats.get_stats().await.context(error_selector)?;
    let previous_recordings = previous_stats.get_stats().await.context(error_selector)?;

    Ok(generic_top_report(current_recordings, previous_recordings).await)
}

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum TopReportError {
    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the artist statistics
    ArtistStatsError {
        source: ArtistStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the release statistics
    ReleaseStatsError {
        source: ReleaseStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the release statistics
    ReleaseGroupStatsError {
        source: ReleaseGroupStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the release statistics
    LabelStatsError {
        source: LabelStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for TopReportError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RecordingStatsError { .. } => None,
            Self::ArtistStatsError { .. } => None,
            Self::ReleaseStatsError { .. } => None,
            Self::ReleaseGroupStatsError { .. } => None,
            Self::LabelStatsError { .. } => None,
        }
    }
}
