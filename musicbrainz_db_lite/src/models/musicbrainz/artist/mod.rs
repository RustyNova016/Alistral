pub mod relations;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use musicbrainz_rs::entity::artist::Artist as MBArtist;
use sequelles::has_rowid::HasRowID;
use sqlx::prelude::FromRow;

use crate::MBIDRedirection;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::hardlink_methods::impl_db_relation_fetch_methods;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

use super::relations::impl_relations::impl_relations;

pub mod crawler;
pub mod display;
pub mod finds;

#[derive(Debug, Default, Clone, PartialEq, Eq, FromRow, Upsert, MainEntity)]
#[database(
    table = "artists",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct Artist {
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub sort_name: String,
    pub disambiguation: String,
    pub country: Option<String>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_get_and_fetch!(Artist);
impl_relations!(Artist);

impl_db_relation_methods!(Artist);
impl_db_relation_fetch_methods!(Artist, MBArtist);

impl HasTags for Artist {}
impl HasGenres for Artist {}
impl MBIDRedirection for Artist {}

impl HasRowID for Artist {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl crate::RowId for Artist {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for Artist {
    const TABLE_NAME: &str = "artists";
    const FOREIGN_FIELD_NAME: &str = "artist";
}
