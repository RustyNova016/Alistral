use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use alistral_core::models::listen_statistics_data::artists::ArtistStatsError;
use alistral_core::models::listen_statistics_data::labels::LabelStatsError;
use alistral_core::models::listen_statistics_data::release::ReleaseStatsError;
use alistral_core::models::listen_statistics_data::release_groups::ReleaseGroupStatsError;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum SectionsError {
    /// Error while getting the artist statistics
    ArtistStatsError {
        source: ArtistStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the label statistics
    LabelStatsError {
        source: LabelStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the release statistics
    ReleaseStatsError {
        source: ReleaseStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    /// Error while getting the release group statistics
    ReleaseGroupStatsError {
        source: ReleaseGroupStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for SectionsError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::ArtistStatsError { .. } => None,
            Self::LabelStatsError { .. } => None,
            Self::RecordingStatsError { .. } => None,
            Self::ReleaseStatsError { .. } => None,
            Self::ReleaseGroupStatsError { .. } => None,
        }
    }
}
