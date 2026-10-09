use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;

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
