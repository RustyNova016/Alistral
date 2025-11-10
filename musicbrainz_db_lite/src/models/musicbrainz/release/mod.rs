use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

use crate::HasArtistCredits;
use crate::HasMBID;
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
pub mod fetching;
pub mod finds;
pub mod methods;
pub mod relations;
pub mod upsert;

#[derive(Debug, Default, Clone, FromRow, PartialEq, Eq, Deserialize, Serialize)]
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

impl HasTable for Release {
    const TABLE_NAME: &str = "releases";
    const FOREIGN_FIELD_NAME: &str = "release";
}

impl HasMBID for Release {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }
}

impl HasTags for Release {}
impl HasGenres for Release {}
impl MBIDRedirection for Release {}
