pub mod find_by;
pub mod find_by_mbid;
pub mod find_by_rowid;
pub mod has_genre;
pub mod has_table;
pub mod has_tags;

pub trait RowId {
    /// Returns an unique i64 number that identify the row
    fn get_row_id(&self) -> i64;
}

pub trait Upsertable: Sized {
    fn upsert(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send;
}

pub trait HasMBID {
    fn get_mbid(&self) -> &str;
}
