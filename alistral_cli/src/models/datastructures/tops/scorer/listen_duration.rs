use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use chrono::Duration;

use crate::models::datastructures::tops::display_top::DisplayTop;
use crate::models::datastructures::tops::scorer::TopScorer;

pub struct ListenDurationTopScorer;

impl<L> TopScorer<L> for ListenDurationTopScorer
where
    L: ListenCollWithTime,
{
    type Score = Duration;

    fn get_score_of_element(&self, element: L) -> Self::Score {
        element.get_time_listened().unwrap_or_default()
    }
}
