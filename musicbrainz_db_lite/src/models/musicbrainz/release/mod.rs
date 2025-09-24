use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

use crate::HasArtistCredits;
use crate::MBIDRedirection;
use crate::MBRelease;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::hardlink_methods::impl_db_relation_fetch_methods;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

pub mod display;
pub mod finds;
pub mod methods;
pub mod relations;

#[derive(
    Debug, Default, Clone, FromRow, Upsert, MainEntity, PartialEq, Eq, Deserialize, Serialize,
)]
#[database(
    table = "releases",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct Release {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub date: Option<i64>,
    pub country: Option<String>,
    pub quality: Option<String>,
    pub status: Option<String>,

    /// Barcode of the release.
    ///
    /// "" means "This release does not have a barcode" has been checked
    ///
    /// `None` means that the barcode is an empty string
    pub barcode: Option<String>,
    pub disambiguation: Option<String>,
    pub packaging: Option<String>,
    pub annotation: Option<String>,
    pub full_update_date: Option<i64>,

    pub artist_credit: Option<i64>,
    pub release_group: Option<i64>,
}

impl_get_and_fetch!(Release);
impl_relations!(Release);
impl_db_relation_methods!(Release);
impl_db_relation_fetch_methods!(Release, MBRelease);

impl crate::RowId for Release {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasRowID for Release {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasArtistCredits<MBRelease> for Release {
    fn get_artist_credits_id(&self) -> Option<&i64> {
        self.artist_credit.as_ref()
    }

    fn get_title(&self) -> &str {
        &self.title
    }

    fn set_artist_credits_id(&mut self, id: Option<i64>) {
        self.artist_credit = id
    }
}

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(
    table = "medias",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id)
)]
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

impl crate::RowId for Media {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

#[derive(Debug, Default, Clone, FromRow, Upsert)]
#[database(
    table = "label_infos",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, gid)
)]
pub struct LabelInfo {
    pub id: i64,
    pub catalog_number: Option<String>,
    pub label: Option<String>,
    pub release: i64,
}

impl crate::RowId for LabelInfo {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for Release {
    const TABLE_NAME: &str = "releases";
    const FOREIGN_FIELD_NAME: &str = "release";
}

impl HasTags for Release {}
impl HasGenres for Release {}
impl MBIDRedirection for Release {}
