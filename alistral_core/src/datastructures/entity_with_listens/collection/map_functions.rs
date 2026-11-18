use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;

impl<Ent, Lis> EntityWithListensCollection<Ent, Lis> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = &EntityWithListens<Ent, Lis>> {
        self.0.values()
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = &Ent> {
        self.0.values().map(|r| &r.entity)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<Ent, Lis> Default for EntityWithListensCollection<Ent, Lis> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Ent, Lis> IntoIterator for EntityWithListensCollection<Ent, Lis> {
    type Item = EntityWithListens<Ent, Lis>;
    type IntoIter = IntoValues<i64, Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}
