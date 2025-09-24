use sequelles::has_rowid::HasRowID;
use sqlx::FromRow;

pub mod upsert;

#[derive(Debug, Default, Clone, FromRow)]
pub struct Media {
    pub id: i64,
    pub track_count: i64,
    pub title: Option<String>,
    pub position: Option<i64>,
    pub disc_count: Option<i64>,
    pub format: Option<String>,
    pub track_offset: Option<i64>,

    pub release: i64,
}

impl HasRowID for Media {
    fn rowid(&self) -> i64 {
        self.id
    }
}
