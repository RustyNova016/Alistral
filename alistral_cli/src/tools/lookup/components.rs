use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;

#[derive(bon::Builder)]
pub(super) struct LookupLine<T, L> {
    description: String,
    current_data: EntityWithListensCollection<T, L>
}