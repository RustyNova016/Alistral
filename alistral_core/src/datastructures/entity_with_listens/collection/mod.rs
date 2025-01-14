use std::collections::HashMap;

use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::RowId;

use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::traits::mergable::Mergable;

use super::EntityWithListens;

#[derive(Debug, Clone)]
pub struct EntityWithListensCollection<Ent, Lis>(pub HashMap<i64, EntityWithListens<Ent, Lis>>)
where
    Ent: RowId,
    Lis: ListenCollectionReadable;

impl<Ent, Lis> EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // --- Getters ---

    pub fn get_by_id(&self, id: i64) -> Option<&EntityWithListens<Ent, Lis>> {
        self.0.get(&id)
    }

    // --- Inserts ---

    /// Insert a EntityWithListens, and merge if it doesn't exists
    pub fn insert_or_merge(&mut self, value: EntityWithListens<Ent, Lis>)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
    {
        self.0
            .entry(value.get_row_id())
            .and_modify(|val| val.merge(value.clone()))
            .or_insert(value);
    }

    pub fn insert_or_merge_listen(&mut self, entity: Ent, listen: Listen)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
        Lis: From<Listen>,
    {
        self.insert_or_merge(EntityWithListens {
            entity,
            listens: listen.into(),
        });
    }
}

impl<Ent, Lis> Default for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn default() -> Self {
        Self::new()
    }
}
