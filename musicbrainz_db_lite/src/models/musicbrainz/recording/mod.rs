pub mod display;
use chrono::Duration;
use musicbrainz_db_lite_macros::{MainEntity, Upsert};
use serde::Deserialize;
use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::{
    artist_credits::impl_artist_credits, get_and_fetch::impl_get_and_fetch, impl_redirections,
};

use super::relations::impl_relations::impl_relations;

pub mod finds;
pub mod relations;
pub mod status;

#[derive(
    Debug, Default, PartialEq, Eq, Clone, FromRow, Upsert, MainEntity, Deserialize, Serialize,
)]
#[database(
    table = "recordings",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
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

impl_redirections!(Recording, "recordings");
impl_artist_credits!(Recording, "recordings");
impl_get_and_fetch!(Recording);
impl_relations!(Recording);

impl crate::RowId for Recording {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl Recording {
    pub fn length_as_duration(&self) -> Option<Duration> {
        self.length.and_then(|length| {
            Duration::new(length.div_euclid(1000), length.rem_euclid(1000) as u32)
        })
    }
}

impl HasTable for Recording {
    const TABLE_NAME: &str = "recordings";
    const FOREIGN_FIELD_NAME: &str = "recording";
}

impl HasTags for Recording {}
impl HasGenres for Recording {}
