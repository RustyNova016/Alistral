use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;


use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::GetFriendlyError;

#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub(super)))]
pub enum WeeklyCommandError {
    /// Error while getting the recording statistics
    RecordingStatsError {
        source: RecordingStatsError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}

impl GetFriendlyError for WeeklyCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::RecordingStatsError { .. } => None,
        }
    }
}
