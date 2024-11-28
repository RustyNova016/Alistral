use std::sync::Arc;

use crate::models::data::listenbrainz::listen::Listen;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use super::ListenCollection;

impl ListenCollection {
    /// ## Safety
    /// - Allows Unmapped
    /// - Doesn't check ids
    pub fn get_listens_of_recording(&self, recording: &RecordingMBID) -> Self {
        self.iter()
            .filter(|listen| {
                listen
                    .get_mapping_data()
                    .as_ref()
                    .is_some_and(|mapping| mapping.recording_mbid == recording.to_string())
                // TODO: Prevent Recording MBID cast
            })
            .cloned()
            .collect()
    }

}
