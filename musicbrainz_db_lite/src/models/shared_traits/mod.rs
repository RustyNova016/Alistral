pub mod completeness;
pub mod db_relation;
pub mod fetch_and_save;
pub mod fetch_mbid;
pub mod find_by;
pub mod find_by_mbid;
pub mod find_by_rowid;
pub mod get_or_fetch;
pub mod has_artist_credits;
pub mod has_genre;
pub mod has_table;
pub mod has_tags;
pub mod has_urls;
pub mod mbid_redirection;
pub mod save_from;

pub trait Upsertable: Sized {
    fn upsert(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send;
}

pub trait HasMBID {
    fn get_mbid(&self) -> &str;
}
