use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use snafu::Snafu;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum DailyCommandError {
    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for DailyCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RecordingStatsError {
                source: _,
                location: _,
            } => None,
        }
    }
}
