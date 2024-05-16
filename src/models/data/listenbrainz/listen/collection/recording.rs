use std::sync::Arc;

use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;

impl ListenCollection {
    pub fn get_listens_of_recording(&self, recording: &RecordingMBID) -> Self {
        self.iter()
            .filter(|listen| {
                listen
                    .get_mapping_data()
                    .as_ref()
                    .is_some_and(|mapping| mapping.recording_mbid == recording.to_string())
                // TODO: Prevent Recording MSID cast
            })
            .cloned()
            .collect()
    }

    pub fn get_latest_listen_of_recording(&self, recording: &RecordingMBID) -> Option<Arc<Listen>> {
        self.get_listens_of_recording(recording).get_latest_listen()
    }
}
