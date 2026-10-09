use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::listen_collection::ListenCollection;

impl ListenCollection {
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Listen> {
        self.data.iter_mut()
    }
}
