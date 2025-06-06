use core::cmp::Reverse;
use core::fmt::Debug;
use std::collections::HashMap;
use std::collections::hash_map::IntoValues;

use futures::Stream;
use futures::stream;
use itertools::Itertools as _;
use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use rust_decimal::Decimal;

use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::listen_sorter::ListenSortingStrategy;
use crate::traits::mergable::Mergable;

use super::EntityWithListens;

pub mod converters;

/// An indexed collection of [`EntityWithListens`]
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

    // --- Iters ---

    pub fn iter(&self) -> impl Iterator<Item = &EntityWithListens<Ent, Lis>> {
        self.0.values()
    }

    pub fn iter_entities(&self) -> impl Iterator<Item = &Ent> {
        self.0.values().map(|r| &r.entity)
    }

    pub fn into_stream(self) -> impl Stream<Item = EntityWithListens<Ent, Lis>> {
        stream::iter(self)
    }

    // --- Inserts ---

    /// Insert a EntityWithListensCollection, and merge if it doesn't exists
    pub fn insert_or_merge(&mut self, value: Self)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
    {
        for entity in value.into_iter() {
            self.insert_or_merge_entity_stats(entity);
        }
    }

    /// Insert a EntityWithListens, and merge if it doesn't exists
    pub fn insert_or_merge_entity_stats(&mut self, value: EntityWithListens<Ent, Lis>)
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
        self.insert_or_merge_entity_stats(EntityWithListens {
            entity,
            listens: listen.into(),
        });
    }

    pub fn insert_or_merge_listens(&mut self, entity: Ent, listens: Vec<Listen>)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
        Lis: From<Vec<Listen>>,
    {
        self.insert_or_merge_entity_stats(EntityWithListens {
            entity,
            listens: listens.into(),
        });
    }

    /// Insert a Entity with zero listens, and merge if it doesn't exists
    pub fn insert_or_merge_entity(&mut self, entity: Ent)
    where
        EntityWithListens<Ent, Lis>: Mergable + Clone,
        Lis: Default,
    {
        self.insert_or_merge_entity_stats(EntityWithListens {
            entity,
            listens: Default::default(),
        });
    }

    // --- Stats ---

    /// Return the ratio of total listens of the entity collection being from a specific entity
    pub fn get_listen_ratio(&self, entity: &Ent) -> Decimal {
        let recording_listen_count = self
            .get_by_id(entity.get_row_id())
            .map(|r| r.listen_count())
            .unwrap_or(0);

        Decimal::new(recording_listen_count.try_into().unwrap(), 0)
            / Decimal::new(self.listen_count().try_into().unwrap(), 0)
    }

    /// Return the rank of the listened entity
    pub fn get_rank(&self, entity: &Ent) -> Option<usize> {
        let mut rec_ref = self.iter().collect_vec();
        rec_ref.sort_by_cached_key(|rec| Reverse(rec.listen_count()));

        let mut rank = 1;
        let mut last_listen_count = 0;

        for (i, rec) in rec_ref.iter().enumerate() {
            if last_listen_count != rec.listen_count() {
                rank = i;
            }

            if rec.entity().get_row_id() == entity.get_row_id() {
                return Some(rank);
            }

            last_listen_count = rec.listen_count();
        }

        None
    }

    /// Insert a listen with a specific sorting strategy
    pub async fn insert_listen_with<T>(
        &mut self,
        listen: Listen,
        strategy: &T,
    ) -> Result<(), crate::Error>
    where
        T: ListenSortingStrategy<Ent, Lis>,
    {
        strategy.sort_insert_listen(self, listen).await
    }

    /// Insert a collection of listens with a specific sorting strategy
    pub async fn insert_listens_with<T>(
        &mut self,
        listens: Vec<Listen>,
        strategy: &T,
    ) -> Result<(), crate::Error>
    where
        T: ListenSortingStrategy<Ent, Lis>,
    {
        strategy.sort_insert_listens(self, listens).await
    }

    pub async fn from_listens<S>(listens: Vec<Listen>, strat: &S) -> Result<Self, crate::Error>
    where
        S: ListenSortingStrategy<Ent, Lis>,
    {
        let mut new = Self::new();
        new.insert_listens_with(listens, strat).await?;
        Ok(new)
    }

    pub async fn from_listencollection<S>(
        listens: ListenCollection,
        strat: &S,
    ) -> Result<Self, crate::Error>
    where
        S: ListenSortingStrategy<Ent, Lis>,
    {
        Self::from_listens(listens.data, strat).await
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

impl<Ent, Lis> ListenCollectionReadable for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.iter().flat_map(|lis| lis.iter_listens())
    }
}

impl<Ent, Lis> Mergable for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
    EntityWithListens<Ent, Lis>: Mergable + Clone,
{
    fn merge(&mut self, other: Self) {
        self.insert_or_merge(other)
    }
}

impl<Ent, Lis> IntoIterator for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    type Item = EntityWithListens<Ent, Lis>;
    type IntoIter = IntoValues<i64, Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_values()
    }
}
