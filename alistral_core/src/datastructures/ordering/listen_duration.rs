use crate::datastructures::entity_with_listens::traits::ListenCollWithTime;
use crate::datastructures::ordering::Orderer;

/// Order the listens by listened duration
pub struct ListenDurationOrdering;

impl<T> Orderer<T> for ListenDurationOrdering
where
    T: ListenCollWithTime,
{
    fn order(&self, mut items: Vec<T>) -> Vec<T> {
        items.sort_by_cached_key(|item| item.get_time_listened());
        items
    }
}
