use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use itertools::Itertools;

use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::cli_components::tables::table::TopTable;

impl<R, S> TopTable<R, S> {
    pub fn from_entity_listens<Ent, Lis>(
        data: impl IntoIterator<Item = EntityWithListens<Ent, Lis>>,
        sort: S,
        reversed: bool,
    ) -> Self
    where
        R: TableRow + From<EntityWithListens<Ent, Lis>>,
        S: TableSort<R>,
    {
        let rows = data
            .into_iter()
            .map(|entity_listens| R::from(entity_listens))
            .collect_vec();

        TopTable::new(rows, sort, reversed, false)
    }

    pub fn from_entity_listens_comps<Ent, Lis>(
        data: impl IntoIterator<Item = EntityListensComparison<Ent, Lis>>,
        sort: S,
        reversed: bool,
    ) -> Self
    where
        R: TableRow + From<EntityListensComparison<Ent, Lis>>,
        S: TableSort<R>,
    {
        let rows = data
            .into_iter()
            .map(|entity_listens| R::from(entity_listens))
            .collect_vec();

        TopTable::new(rows, sort, reversed, true)
    }
}
