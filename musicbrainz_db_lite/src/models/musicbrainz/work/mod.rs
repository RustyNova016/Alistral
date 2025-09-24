use sequelles::has_rowid::HasRowID;
use sqlx::FromRow;

use crate::HasMBID;
use crate::MBIDRedirection;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;

pub mod display;
pub mod finds;
pub mod upsert;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow)]
pub struct Work {
    pub id: i64,
    pub mbid: String,
    pub title: String,
    pub work_type: Option<String>,
    pub disambiguation: Option<String>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_get_and_fetch!(Work);
impl_relations!(Work);

impl HasRowID for Work {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTable for Work {
    const TABLE_NAME: &str = "works";
    const FOREIGN_FIELD_NAME: &str = "work";
}

impl HasMBID for Work {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }
}

impl HasTags for Work {}
impl HasGenres for Work {}
impl MBIDRedirection for Work {}
