use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use futures::stream::BoxStream;

pub type LayerResult<'a> = Result<RadioStream<'a>, crate::Error>;

pub type RadioStream<'a> = BoxStream<'a, RecordingWithListens>;
