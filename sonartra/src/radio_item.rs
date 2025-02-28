use core::ops::Deref;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioItem {
    recording: RecordingWithListens,
    pub score: i64,
}

impl Deref for RadioItem {
    type Target = RecordingWithListens;
    fn deref(&self) -> &Self::Target {
        &self.recording
    }
}
