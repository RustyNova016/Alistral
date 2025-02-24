pub mod finds;
pub mod relations;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sqlx::FromRow;

use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::artist_credits::impl_artist_credits;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::impl_redirections;

use super::relations::impl_relations::impl_relations;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Upsert, MainEntity)]
#[database(
    table = "release_groups",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct ReleaseGroup {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub primary_type_id: Option<String>,
    pub first_release_date: Option<i64>,
    pub disambiguation: String,
    pub annotation: Option<String>,

    pub artist_credit: Option<i64>,

    pub full_update_date: Option<i64>,
}

impl_redirections!(ReleaseGroup, "release_groups");
impl_get_and_fetch!(ReleaseGroup);
impl_artist_credits!(ReleaseGroup, "release_groups");
impl_relations!(ReleaseGroup);

impl crate::RowId for ReleaseGroup {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for ReleaseGroup {
    const TABLE_NAME: &str = "release_groups";
    const FOREIGN_FIELD_NAME: &str = "release_group";
}

impl HasTags for ReleaseGroup {}
impl HasGenres for ReleaseGroup {}
