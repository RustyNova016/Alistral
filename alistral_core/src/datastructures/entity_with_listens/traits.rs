use chrono::TimeDelta;

use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;

/// Trait for all the listen Collections that can return the total time listened
pub trait ListenCollWithTime {
    /// Return the total listens time all the listens in the collection
    fn get_time_listened(&self) -> Option<TimeDelta>;
}

pub trait FromListenCollection
where
    Self: Default,
{
    fn from_listencollection(
        client: &crate::AlistralClient,
        listens: ListenCollection,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send;
}

/// Get an iterator of [`RecordingWithListens`]
pub trait IterRecordingWithListens {
    /// Get an iterator of [`RecordingWithListens`]
    fn iter_recording_with_listens(&self) -> impl Iterator<Item = &RecordingWithListens>;
}
