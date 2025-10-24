use alistral_core::cli::colors::AlistralColors;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub struct TopRow<S> where S: Ord + Eq {
    pub ranking: usize,
    pub previous_ranking: Option<usize>,

    pub score: TopScore<S>,
    pub previous_score: Option<TopScore<S>>,

    pub element: Box<dyn FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>>,
}

impl<S> TopRow<S> where S: Ord + Eq {
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
            None => format!("{}", self.score),
            Some(previous_score) => format!(
                "{} {:<4} {}",
                ComparisonArrow::greater_is_better(&self.score, &previous_score),
                self.score,
                format!("≪ {:>4}", previous_score).true_color_tup((100, 100, 100))
            ),
        }
    }

    pub async fn element_col(&self) -> String {
        self.element.format_with_async(&LISTENBRAINZ_FMT).await.unwrap()
    }
}
