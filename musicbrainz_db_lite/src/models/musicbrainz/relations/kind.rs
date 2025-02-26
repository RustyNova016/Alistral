use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::relations::Relation;

impl Relation<Recording, Recording> {
    /// Return true if `entity_1` is a remix of the provided recording (`entity0`)
    pub fn is_remix_of_rel(&self, base_recording: &Recording) -> bool {
        self.relation_type == "remix"
            && self.entity0 == base_recording.id
            && self.direction == "forward"
    }
}

impl Relation<Recording, Artist> {
    /// Return true if the artist created the remix (the provided recording)
    pub fn is_remixer_rel(&self, base_recording: &Recording) -> bool {
        self.relation_type == "remixer"
            && self.entity1 == base_recording.id
            && self.direction == "backward"
    }
}
