pub mod iterator;
use std::collections::HashMap;

use musicbrainz_db_lite::HasRowID;

use crate::datastructures::entity_with_listens::EntityWithListens;
use crate::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;

#[derive(Debug, Clone, Default)]
pub struct EntityListensComparisonCollection<Ent, Lis> {
    pub(super) data: HashMap<i64, EntityListensComparison<Ent, Lis>>,
}

impl<Ent, Lis> EntityListensComparisonCollection<Ent, Lis> {
    pub fn insert_current_iter(
        &mut self,
        elements: impl IntoIterator<Item = EntityWithListens<Ent, Lis>>,
    ) where
        Ent: HasRowID,
    {
        for element in elements {
            self.insert_current(element);
        }
    }

    pub fn insert_current(&mut self, element: EntityWithListens<Ent, Lis>)
    where
        Ent: HasRowID,
    {
        let row_id = element.entity().rowid();
        self.data.entry(row_id).or_default().set_current(element);
    }

    pub fn insert_previous_iter(
        &mut self,
        elements: impl IntoIterator<Item = EntityWithListens<Ent, Lis>>,
    ) where
        Ent: HasRowID,
    {
        for element in elements {
            self.insert_previous(element);
        }
    }

    pub fn insert_previous(&mut self, element: EntityWithListens<Ent, Lis>)
    where
        Ent: HasRowID,
    {
        let row_id = element.entity().rowid();
        self.data.entry(row_id).or_default().set_previous(element);
    }
}
