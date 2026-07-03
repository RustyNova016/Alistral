use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::listen_collection::ListenCollection;

impl ListenCollection {
    pub fn find_by_timestamp(&self, timestamp: i64) -> impl Iterator<Item = &Listen> {
        self.iter().filter(move |l| l.listened_at == timestamp)
    }
}
