use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;

impl<Ent, Lis> ListenCollectionReadable for EntityWithListensCollection<Ent, Lis>
where
    Lis: ListenCollectionReadable,
{
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.iter().flat_map(|lis| lis.iter_listens())
    }
}
