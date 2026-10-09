use crate::datastructures::entity_with_listens::recording::RecordingWithListens;

impl RecordingWithListens {
    /// Set the duration of the listen to the recording's. If the listen duration is already set, it won't overwrite it unless overwrite is set to true
    pub fn set_listens_duration(&mut self, overwrite: bool) {
        for listen in self.listens.iter_mut() {
            if overwrite {
                listen.duration = self.entity.length.or(listen.duration)
            } else {
                listen.duration = listen.duration.or(self.entity.length)
            }
        }
    }
}
