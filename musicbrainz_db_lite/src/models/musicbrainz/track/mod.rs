pub mod crawler;
pub mod display;
pub mod fetching;
pub mod find;
pub mod relations;
use musicbrainz_db_lite_macros::Upsert;
use sequelles::has_rowid::HasRowID;
use sqlx::FromRow;

use crate::HasMBID;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

#[derive(Debug, Default, Clone, FromRow, Upsert, PartialEq, Eq)]
#[database(
    table = "tracks",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, gid)
)]
pub struct Track {
    pub id: i64,
    pub gid: String,
    pub title: String,
    pub number: String,
    pub length: Option<i64>,
    pub position: i64,

    pub media: i64,
    pub recording: Option<i64>,
    pub artist_credit: Option<i64>,
}

// impl_artist_credits!(Track, "tracks");
impl_db_relation_methods!(Track);

impl crate::RowId for Track {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasRowID for Track {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasMBID for Track {
    fn get_mbid(&self) -> &str {
        &self.gid
    }
}

impl HasTable for Track {
    const TABLE_NAME: &str = "tracks";
    const FOREIGN_FIELD_NAME: &str = "track";
}
