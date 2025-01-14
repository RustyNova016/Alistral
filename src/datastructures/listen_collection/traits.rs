use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use super::ListenCollection;

pub trait ListenCollectionLike {
    /// Return the number of listens in the collection
    fn listen_count(&self) -> usize {
        self.iter_listens().unique_by(|l| l.id).collect_vec().len()
    }

    fn has_no_listens(&self) -> bool {
        self.listen_count() == 0
    }

    /// Iterate over all the listens. They may not be deduplicated
    fn iter_listens(&self) -> impl Iterator<Item = &Listen>;
}

impl ListenCollectionLike for ListenCollection {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.data.iter()
    }
}
