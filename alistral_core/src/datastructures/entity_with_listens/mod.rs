pub mod messybrainz;
pub mod release_group;
use chrono::Duration;
use chrono::Utc;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::RowId;

use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::datastructures::entity_with_listens::traits::IterRecordingWithListens;
use crate::traits::mergable::Mergable;

use super::listen_collection::traits::ListenCollectionReadable;
use super::listen_collection::ListenCollection;

pub mod artist;
pub mod collection;
pub mod entity_as_listens;
pub mod recording;
pub mod release;
pub mod statistic_data;
pub mod traits;
pub mod work;

/// A structure representing an entity with associated listens.
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
    pub fn new(entity: Ent, listens: Lis) -> Self {
        Self { entity, listens }
    }

    pub fn entity(&self) -> &Ent {
        &self.entity
    }

    pub fn listens(&self) -> &Lis {
        &self.listens
    }

    /// Return the amount of time this entity having known about (Since first associated listen)
    pub fn known_for(&self) -> Option<Duration> {
        self.oldest_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }
}

impl<Ent, Lis> ListenCollectionReadable for EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
{
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.listens.iter_listens()
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

impl<Ent, Lis> Mergable for EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable + Mergable,
{
    fn merge(&mut self, other: Self) {
        if self.entity.get_row_id() != other.entity.get_row_id() {
            #[cfg(debug_assertions)] // This is an awkward situation. Let's crash in debug to catch those cases
            panic!("Tried to merge two different recordings");

            #[cfg(not(debug_assertions))]
            return;
        }

        self.listens.merge(other.listens);
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

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for ListenCollection
where
    Ent: RowId,
    Lis: IntoIterator<Item = Listen> + ListenCollectionReadable,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        value.listens.into_iter().collect()
    }
}

impl<Ent, Lis> IterRecordingWithListens for EntityWithListens<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable + IterRecordingWithListens,
{
    fn iter_recording_with_listens(&self) -> impl Iterator<Item = &RecordingWithListens> {
        self.listens.iter_recording_with_listens()
    }
}
