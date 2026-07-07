pub mod cells;
pub mod order_by;
pub mod rows;
pub mod table;

pub trait TableSort<Row> {
    fn get_row_score(&self, row: &Row) -> impl Ord;
    fn get_row_prev_score(&self, row: &Row) -> Option<impl Ord>;
}
