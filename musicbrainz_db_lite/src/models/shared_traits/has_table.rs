/// Traits for all the entities that are contained withing a table
pub trait HasTable {
    const TABLE_NAME: &str;
    const FOREIGN_FIELD_NAME: &str;
}
