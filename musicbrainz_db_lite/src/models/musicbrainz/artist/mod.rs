use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::prelude::FromRow;

use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::impl_redirections;

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

impl_redirections!(Artist, "artists");
impl_get_and_fetch!(Artist);
impl_relations!(Artist);
//impl_fetch_by_mbid!(Artist);

impl crate::RowId for Artist {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for Artist {
    const TABLE_NAME: &str = "artists";
    const FOREIGN_FIELD_NAME: &str = "artist";
}

impl HasTags for Artist {}
impl HasGenres for Artist {}
impl crate::MBIDRedirection for Artist {}
