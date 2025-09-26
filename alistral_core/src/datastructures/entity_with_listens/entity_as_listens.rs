use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID;
use rust_decimal::Decimal;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::listen_collection::traits::ListenCollectionReadable;

impl<EntMain, EntSub, Lis> EntityWithListens<EntMain, EntityWithListensCollection<EntSub, Lis>>
where
    EntMain: HasRowID,
    EntSub: HasRowID,
    Lis: ListenCollectionReadable,
{
    /// Return the number of child entities that have been listened
    fn listened_entity_count(&self) -> usize {
        self.listens
            .iter()
            .filter(|r| r.listen_count() != 0)
            .collect_vec()
            .len()
    }

    /// Return the average listens count per each recording listened by the user
    pub fn average_listen_per_entity_listened(&self) -> Decimal {
        Decimal::from(self.listen_count()) / Decimal::from(self.listened_entity_count())
    }
}
