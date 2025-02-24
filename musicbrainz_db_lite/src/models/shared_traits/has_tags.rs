use super::has_table::HasTable;

pub trait HasTags: HasTable {
    fn get_tag_table_name(&self) -> String {
        format!("{}_tags", Self::TABLE_NAME)
    }
}
