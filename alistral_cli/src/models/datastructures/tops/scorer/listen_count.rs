use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;

use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_score::TopScore;

pub struct ListenCountTopScorer;

impl<L> TopScorer<L> for ListenCountTopScorer
where
    L: ListenCollectionReadable,
{
    fn get_score_of_element(&self, element: &L) -> TopScore {
        TopScore::Number(element.listen_count() as i64)
    }
}
