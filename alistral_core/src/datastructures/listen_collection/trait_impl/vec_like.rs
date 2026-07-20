use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::listen_collection::ListenCollection;
use crate::traits::vec_like::VecLike;

impl VecLike<Listen> for ListenCollection {
    fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Listen) -> bool,
    {
        self.data.retain(f);
    }
}
