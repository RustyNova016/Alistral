use alistral_core::cli::colors::AlistralColors;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::datastructures::tops::display_top::DisplayTop;

pub struct TopRow<T> {
    pub ranking: usize,
    pub previous_ranking: Option<usize>,

    pub score: Box<dyn DisplayTop>,
    pub previous_score: Option<Box<dyn DisplayTop>>,

    pub element: T,
}

impl<T> TopRow<T> {
    pub fn rank_col(&self) -> String {
        match self.previous_ranking {
            None => format!("#{}", self.ranking),
            Some(previous_ranking) => format!(
                "{} #{:<3} {}", //←⇐⇦⇽≪
                ComparisonArrow::lesser_is_better_inv(self.ranking, previous_ranking),
                self.ranking,
                format!("≪ {previous_ranking:>3}").true_color_tup((100, 100, 100))
            ),
        }
    }

    pub fn score_col(&self) -> String {
        match &self.previous_score {
            None => format!("{}", self.score.to_string_top()),
            Some(previous_score) => format!(
                "{:<4} {}",
                self.score.to_string_top(),
                format!("≪ {:>4}", previous_score.to_string_top()).true_color_tup((100, 100, 100))
            ),
        }
    }
}
