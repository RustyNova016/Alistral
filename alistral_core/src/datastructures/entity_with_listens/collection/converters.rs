//! Implementations for type convertions

use musicbrainz_db_lite::RowId;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::RecordingWithListens;
use crate::datastructures::entity_with_listens::traits::IterRecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::traits::mergable::Mergable;

impl<Ent, Lis> From<Vec<EntityWithListens<Ent, Lis>>> for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
    EntityWithListens<Ent, Lis>: Mergable + Clone,
{
    fn from(value: Vec<EntityWithListens<Ent, Lis>>) -> Self {
        let mut new = Self::default();

        for ent in value {
            new.insert_or_merge_entity_stats(ent);
        }

        new
    }
}

impl<Ent, Lis> From<EntityWithListens<Ent, Lis>> for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable,
    EntityWithListens<Ent, Lis>: Mergable + Clone,
{
    fn from(value: EntityWithListens<Ent, Lis>) -> Self {
        let mut new = Self::default();
        new.insert_or_merge_entity_stats(value);
        new
    }
}

impl<Ent, Lis> From<EntityWithListensCollection<Ent, Lis>> for ListenCollection
where
    Ent: RowId,
    Lis: ListenCollectionReadable + IntoIterator<Item = Listen>,
{
    fn from(value: EntityWithListensCollection<Ent, Lis>) -> Self {
        value.into_iter().flat_map(ListenCollection::from).collect()
    }
}

impl<Ent, Lis> IterRecordingWithListens for EntityWithListensCollection<Ent, Lis>
where
    Ent: RowId,
    Lis: ListenCollectionReadable + IterRecordingWithListens,
{
    fn iter_recording_with_listens(&self) -> impl Iterator<Item = &RecordingWithListens> {
        self.0
            .values()
            .flat_map(|value| value.listens.iter_recording_with_listens())
    }
}
