pub mod top_listen_row;
pub mod top_listen_dur_row;

pub trait TableRow: Sized {
    fn get_table_header() -> Vec<String>;

    async fn format(&self, rank: usize) -> comfy_table::Row;
}