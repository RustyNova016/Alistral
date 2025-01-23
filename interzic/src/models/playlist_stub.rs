use crate::models::messy_recording::MessyRecording;

#[derive(Debug, Clone)]
pub struct PlaylistStub {
    pub title: String,
    pub description: String,
    pub recordings: Vec<MessyRecording>,
}