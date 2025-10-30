use crate::models::datastructures::tops::top_score::TopScore;

pub mod ent_with_listens;
pub mod listen_count;
pub mod listen_duration;


/// Trait for all the scorers for the top
pub trait TopScorer<T> {
    fn get_score_of_element(&self, element: &T) -> TopScore;
}

