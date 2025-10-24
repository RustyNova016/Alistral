use crate::models::datastructures::tops::top_score::TopScore;

pub mod listen_count;
pub mod listen_duration;

/// Trait for all the scorers for the top
pub trait TopScorer<T> {
    type Score: Ord;

    fn get_score_of_element(&self, element: &T) -> TopScore<Self::Score>;
}
