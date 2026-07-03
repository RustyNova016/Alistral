pub mod order_by;
pub mod table;
pub mod rows;
pub mod cells;

pub trait TableSort<Row> {
    fn get_row_score(&self, row: &Row) -> impl Ord;
}