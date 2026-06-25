use crate::datastructures::entity_with_listens::entity_comparison::EntityListensComparison;

impl<Ent, Lis> Default for EntityListensComparison<Ent, Lis> {
    fn default() -> Self {
        Self {
            current: None,
            previous: None,
        }
    }
}
