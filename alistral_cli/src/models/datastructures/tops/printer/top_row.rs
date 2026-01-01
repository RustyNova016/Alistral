use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::datastructures::tops::printer::top_cell::TopCell;
use crate::utils::constants::{LISTENBRAINZ_FMT, YIM_FMT};

pub struct TopRow {
    pub ranking: Option<TopCell<usize>>,

    pub listen_count: Option<TopCell<usize>>,

    pub listen_duration: Option<TopCell<HumanTimePrinter>>,

    pub element:
        Box<dyn FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>>,
}

impl TopRow {
    pub fn rank_col(&self) -> String {
        let Some(cell) = &self.ranking else {
            return "Missing stats".to_string();
        };

        if !cell.show_prev {
            cell.format(4, 4)
        } else {
            format!(
                "{} {}",
                ComparisonArrow::lesser_is_better_inv(&cell.current, &cell.previous),
                cell.format(4, 4)
            )
        }
    }

    pub fn score_col(&self) -> String {
        let Some(cell) = &self.listen_duration else {
            return "Missing stats".to_string();
        };

        if !cell.show_prev {
            cell.format(4, 4)
        } else {
            format!(
                "{} {}",
                ComparisonArrow::lesser_is_better_inv(&cell.current, &cell.previous),
                cell.format(4, 4)
            )
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

    pub async fn element_col(&self, with_duration: bool) -> String {
        self.element
            .format_with_async(if with_duration {
                &YIM_FMT
            } else {
                &LISTENBRAINZ_FMT
            })
            .await
            .unwrap()
    }
}
