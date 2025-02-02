use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioItem {
    recording: RecordingWithListens,
    score: i64,
}
