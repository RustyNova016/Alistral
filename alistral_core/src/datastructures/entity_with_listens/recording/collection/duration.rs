use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;

impl RecordingWithListensCollection {
    /// Set the duration of the listen to the recording's. If the listen duration is already set, it won't overwrite it unless overwrite is set to true
    pub fn set_listens_duration(&mut self, overwrite: bool) {
        for stat in self.0.values_mut() {
            stat.set_listens_duration(overwrite);
        }
    }
}
