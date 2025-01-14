use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::RowId;

use crate::traits::mergable::Mergable;

use super::listen_collection::traits::ListenCollectionReadable;
use super::listen_collection::ListenCollection;

pub mod collection;
pub mod recording;
pub mod work;

/// A structure representing an entity with associated listens.
///
/// # Fields
///
/// * `entity` - The entity of type `Ent`.
/// * `listens` - The listens associated with the entity of type `Lis`.
#[derive(Debug, Clone)]
pub struct EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    entity: Ent,
    listens: Lis,
}

impl<Ent, Lis> EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    pub fn listens(&self) -> &Lis {
        &self.listens
    }
}

impl<Ent> EntityWithListens<Ent, ListenCollection>
where
    Ent: RowId,
{
    /// Add a listen if it doesn't already exist in the collection. This doesn't check if the listen belong to the entity
    pub fn insert_unique_listens_unchecked(&mut self, new_listen: Listen) {
        self.listens.push_unique(new_listen);
    }
}

impl<Ent> Mergable for EntityWithListens<Ent, ListenCollection>
where
    Ent: RowId,
{
    fn merge(&mut self, other: Self) {
        if self.entity.get_row_id() != other.entity.get_row_id() {
            #[cfg(debug_assertions)] // This is an awkward situation. Let's crash in debug to catch those cases
            panic!("Tried to merge two different recordings");

            #[cfg(not(debug_assertions))]
            return;
        }

        self.listens.merge_by_index(other.listens);
    }
}

impl<Ent, Lis> RowId for EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn get_row_id(&self) -> i64 {
        self.entity.get_row_id()
    }
}
