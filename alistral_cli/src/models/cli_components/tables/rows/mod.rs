pub mod top_listen_dur_count;
use crate::models::cli_components::tables::cells::rank_cell::RankCell;

pub mod top_listen_duration_row;
pub mod top_listen_row;

pub trait TableRow: Sized {
    fn get_table_header() -> Vec<String>;

    async fn format(&self, rank: RankCell, show_prev: bool) -> comfy_table::Row;

    fn get_row_id(&self) -> i64;
}
