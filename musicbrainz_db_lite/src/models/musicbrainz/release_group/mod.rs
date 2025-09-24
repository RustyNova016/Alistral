use musicbrainz_db_lite_macros::MainEntity;
use sequelles::has_rowid::HasRowID;
use sqlx::FromRow;

use crate::HasArtistCredits;
use crate::MBIDRedirection;
use crate::MBReleaseGroup;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::hardlink_methods::impl_db_relation_fetch_methods;

pub mod display;
pub mod finds;
pub mod relations;
pub mod upsert;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, MainEntity)]
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

impl_get_and_fetch!(ReleaseGroup);
impl_relations!(ReleaseGroup);
impl_db_relation_fetch_methods!(ReleaseGroup, MBReleaseGroup);

impl crate::RowId for ReleaseGroup {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasRowID for ReleaseGroup {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTable for ReleaseGroup {
    const TABLE_NAME: &str = "release_groups";
    const FOREIGN_FIELD_NAME: &str = "release_group";
}

impl HasArtistCredits<MBReleaseGroup> for ReleaseGroup {
    fn get_title(&self) -> &str {
        &self.title
    }

    fn get_artist_credits_id(&self) -> Option<&i64> {
        self.artist_credit.as_ref()
    }

    fn set_artist_credits_id(&mut self, id: Option<i64>) {
        self.artist_credit = id
    }
}

impl HasTags for ReleaseGroup {}
impl HasGenres for ReleaseGroup {}
impl MBIDRedirection for ReleaseGroup {}
