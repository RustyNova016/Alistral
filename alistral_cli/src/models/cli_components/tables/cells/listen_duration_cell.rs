use crate::datastructures::formaters::human_time::HumanTimePrinter;
use crate::models::cli_components::comp_arrow::ComparisonArrow;
use crate::models::cli_components::tables::cells::top_cell::TopCell;

#[derive(Debug, Default)]
pub struct ListenDurationCell(pub TopCell<HumanTimePrinter>);

impl ListenDurationCell {
    pub fn format(&self, max_len_cur: usize, max_len_prev: usize, show_prev: bool) -> String {
        if !show_prev {
            self.0.format(max_len_cur, max_len_prev, show_prev)
        } else {
            format!(
                "{} {}",
                ComparisonArrow::greater_is_better(&self.0.current, &self.0.previous),
                self.0.format(max_len_cur, max_len_prev, show_prev)
            )
        }
    }
}
