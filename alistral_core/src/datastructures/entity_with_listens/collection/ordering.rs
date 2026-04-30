use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;

use crate::EntityWithListens;
use crate::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::traits::ListenCollWithTime;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::ordering::Orderer;
use crate::traits::ordering::OrderByListenDuration;

impl<Ent, Lis> OrderByListenDuration for EntityWithListensCollection<Ent, Lis>
where
    Ent: HasRowID,
    Lis: ListenCollectionReadable,
    EntityWithListens<Ent, Lis>: ListenCollWithTime,
{
    type Item = EntityWithListens<Ent, Lis>;

    fn order_by_listen_duration(&self) -> Vec<&Self::Item> {
        self.iter()
            .sorted_by_cached_key(|item| item.get_time_listened().unwrap_or_default())
            .collect_vec()
    }
}

impl<Ent, Lis> EntityWithListensCollection<Ent, Lis>
where
    Ent: HasRowID + Clone,
    Lis: ListenCollectionReadable + Clone,
{
    pub fn order<O>(&self, orderer: O) -> Vec<EntityWithListens<Ent, Lis>>
    where
        O: Orderer<EntityWithListens<Ent, Lis>>,
    {
        orderer.order(self.iter().cloned().collect_vec())
    }
}
