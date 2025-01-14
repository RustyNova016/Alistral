use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use super::ListenCollection;

/// Trait implemented for all the entities that hold listens, and those listens can be read
pub trait ListenCollectionReadable {
    /// Return the number of listens in the collection
    fn listen_count(&self) -> usize {
        self.iter_listens().unique_by(|l| l.id).collect_vec().len()
    }

    fn has_no_listens(&self) -> bool {
        self.listen_count() == 0
    }

    /// Iterate over all the listens. They may not be deduplicated
    fn iter_listens(&self) -> impl Iterator<Item = &Listen>;

    // --- Listen getters

    /// Returns the oldest listen in the collection.
    fn get_oldest_listen(&self) -> Option<&Listen> {
        self.iter_listens().min_by_key(|listen| listen.listened_at)
    }

    /// Returns the latest listen in the collection.
    fn get_latest_listen(&self) -> Option<&Listen> {
        self.iter_listens().max_by_key(|listen| listen.listened_at)
    }
}

impl ListenCollectionReadable for ListenCollection {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.data.iter()
    }
}
