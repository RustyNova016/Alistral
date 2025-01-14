use std::collections::HashMap;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::RowId;

use crate::utils::traits::mergable::Mergable;

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

    pub fn insert_or_merge(&mut self, value: EntityWithListens<Ent, Lis>)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
    {
        self.0
            .entry(value.get_row_id())
            .and_modify(|val| val.merge(value.clone()))
            .or_insert(value);
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
