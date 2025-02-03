use crate::models::messy_recording::MessyRecording;

#[derive(Debug, Clone)]
pub struct PlaylistStub {
    pub title: String,
    pub description: String,
    pub recordings: Vec<MessyRecording>,
    //TODO: #521 Allow setting playlist visibility
}
