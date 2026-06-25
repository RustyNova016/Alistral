use std::collections::hash_map::IntoValues;

use crate::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;
use crate::datastructures::entity_with_listens::entity_comparison::collection::EntityListensComparisonCollection;

impl<Ent, Lis> IntoIterator for EntityListensComparisonCollection<Ent, Lis> {
    type Item = EntityListensComparison<Ent, Lis>;
    type IntoIter = IntoValues<i64, EntityListensComparison<Ent, Lis>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_values()
    }
}
