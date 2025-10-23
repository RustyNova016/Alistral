use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

use crate::models::datastructures::tops::scorer::TopScorer;

pub struct ListenCountTopScorer;

impl<L> TopScorer<L> for ListenCountTopScorer
where
    L: ListenCollectionReadable,
{
    type Score = usize;

    fn get_score_of_element(&self, element: L) -> Self::Score {
        element.listen_count()
    }
}
