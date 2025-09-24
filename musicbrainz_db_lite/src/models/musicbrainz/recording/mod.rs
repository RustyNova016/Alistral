use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;

use super::relations::impl_relations::impl_relations;
use crate::HasMBID;
use crate::MBIDRedirection;
use crate::MBRecording;
use crate::models::shared_traits::has_artist_credits::HasArtistCredits;
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
pub mod upsert;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Deserialize, Serialize)]
pub struct Recording {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub video: Option<i64>,
    pub length: Option<i64>,
    pub disambiguation: Option<String>,
    pub annotation: Option<String>,
    pub first_release_date: Option<i64>,

    pub full_update_date: Option<i64>,

    pub artist_credit: Option<i64>,
}

impl_get_and_fetch!(Recording);
impl_relations!(Recording);
impl_db_relation_methods!(Recording);
impl_db_relation_fetch_methods!(Recording, MBRecording);

impl HasTags for Recording {}
impl HasGenres for Recording {}
impl MBIDRedirection for Recording {}

impl crate::RowId for Recording {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasRowID for Recording {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTable for Recording {
    const TABLE_NAME: &str = "recordings";
    const FOREIGN_FIELD_NAME: &str = "recording";
}

impl HasMBID for Recording {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }
}

impl HasArtistCredits<MBRecording> for Recording {
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
