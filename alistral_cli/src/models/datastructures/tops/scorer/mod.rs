use crate::models::datastructures::tops::display_top::DisplayTop;

pub mod listen_count;
pub mod listen_duration;

/// Trait for all the scorers for the top
pub trait TopScorer<T> {
    type Score: DisplayTop + 'static;

    fn get_score_of_element(&self, element: T) -> Self::Score;

    fn box_score(&self, score: Self::Score) -> Box<dyn DisplayTop> {
        Box::new(score)
    }
}
