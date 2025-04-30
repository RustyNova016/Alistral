pub mod display;
pub mod finds;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

pub mod relations;

use crate::MBIDRedirection;
use crate::MBRelease;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::hardlink_methods::impl_db_relation_fetch_methods;
use crate::utils::macros::{
    artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch,
};

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

impl_artist_credits!(Release, "releases");
impl_get_and_fetch!(Release);
impl_relations!(Release);
impl_db_relation_fetch_methods!(Release, MBRelease);

impl crate::RowId for Release {
    fn get_row_id(&self) -> i64 {
        self.id
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

impl_artist_credits!(Track, "tracks");

impl crate::RowId for Track {
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
