use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use chrono::Duration;

use crate::models::datastructures::tops::scorer::TopScorer;
use crate::models::datastructures::tops::top_score::TopScore;

pub struct ListenDurationTopScorer;

impl<L> TopScorer<L> for ListenDurationTopScorer
where
    L: ListenCollWithTime,
{
    type Score = Duration;

    fn get_score_of_element(&self, element: L) -> TopScore<Self::Score> {
        let dur = element.get_time_listened().unwrap_or_default();
        TopScore {
            data: dur,
            display: dur.to_string(),
        }
    }
}
