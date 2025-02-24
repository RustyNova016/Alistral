use super::has_table::HasTable;

pub trait HasGenres: HasTable {
    fn get_genre_table_name(&self) -> String {
        format!("{}_tags", Self::TABLE_NAME)
    }
}
