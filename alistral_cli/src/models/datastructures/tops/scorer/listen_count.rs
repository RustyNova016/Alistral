use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_score::TopScore;

pub struct ListenCountTopScorer;

impl<L> TopScorer<L> for ListenCountTopScorer
where
    L: ListenCollectionReadable,
{
    type Score = usize;

    fn get_score_of_element(&self, element: &L) -> TopScore<Self::Score> {
        let count = element.listen_count();
        TopScore { data: count, display: count.to_string() }
    }
}
