use alistral_core::cli::colors::AlistralColors;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::datastructures::tops::printer::top_cell::TopCell;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub struct TopRow {
    pub ranking: usize,
    pub previous_ranking: Option<usize>,

    pub score: TopScore,
    pub previous_score: Option<TopScore>,

    pub listen_count: Option<TopCell<usize>>,
    
    pub element:
        Box<dyn FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>>,
}

impl TopRow {
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
                ComparisonArrow::greater_is_better(&self.score, previous_score),
                self.score,
                format!("≪ {:>4}", previous_score).true_color_tup((100, 100, 100))
            ),
        }
    }

    pub fn listen_col(&self, max_len_cur: usize, max_len_prev: usize) -> String {
        let Some(cell) = &self.listen_count else {
            return "Missing stats".to_string();
        };

        if !cell.show_prev {
            cell.format(max_len_cur, max_len_prev)
        } else {
            format!(
                "{} {}",
                ComparisonArrow::greater_is_better(&cell.current, &cell.previous),
                cell.format(max_len_cur, max_len_prev)
            )
        }
    }

    pub async fn element_col(&self) -> String {
        self.element
            .format_with_async(&LISTENBRAINZ_FMT)
            .await
            .unwrap()
    }
}
