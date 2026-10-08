use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::traits::ListenCollWithTime;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::models::listen_with_time::ListenWithDuration;
use crate::models::listen_with_time::iterator::IntoListenWithDurationIterator;

impl<Ent, Lis> ListenCollectionReadable for EntityWithListensCollection<Ent, Lis>
where
    Lis: ListenCollectionReadable,
{
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.iter().flat_map(|lis| lis.iter_listens())
    }
}

impl<Ent, Lis> IntoListenWithDurationIterator for EntityWithListensCollection<Ent, Lis>
where
    EntityWithListens<Ent, Lis>: IntoListenWithDurationIterator,
{
    fn into_listen_with_duration_iterator(self) -> impl Iterator<Item = ListenWithDuration> {
        self.into_iter()
            .flat_map(|lis| lis.into_listen_with_duration_iterator())
    }
}

impl<Ent, Lis> ListenCollWithTime for EntityWithListensCollection<Ent, Lis>
where
    EntityWithListens<Ent, Lis>: ListenCollWithTime,
{
    fn get_time_listened(&self) -> Option<chrono::TimeDelta> {
        Some(
            self.iter()
                .filter_map(|data| data.get_time_listened())
                .sum(),
        )
    }
}
