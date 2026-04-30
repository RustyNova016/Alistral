use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::ordering::Orderer;

/// Order the listens by listened duration
pub struct ListenDurationOrdering;

impl<T> Orderer<T> for ListenDurationOrdering
where
    T: ListenCollectionReadable,
{
    fn order(&self, mut items: Vec<T>) -> Vec<T> {
        items.sort_by_cached_key(|item| item.listen_count());
        items
    }
}
