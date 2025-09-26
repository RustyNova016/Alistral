use sequelles::has_rowid::HasRowID;
use sqlx::FromRow;

use crate::HasMBID;
use crate::MBIDRedirection;
use crate::MBLabel;
use crate::models::shared_traits::has_genre::HasGenres;
use crate::models::shared_traits::has_table::HasTable;
use crate::models::shared_traits::has_tags::HasTags;
use crate::utils::macros::get_and_fetch::impl_get_and_fetch;
use crate::utils::macros::hardlink_methods::impl_db_relation_fetch_methods;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

use super::relations::impl_relations::impl_relations;

pub mod display;
pub mod finds;
pub mod upsert;

#[derive(Debug, Default, Clone, FromRow, PartialEq, Eq)]
pub struct Label {
    pub id: i64,
    pub mbid: String,
    pub name: String,
    pub label_type: Option<String>,
    pub sort_name: Option<String>,
    pub disambiguation: Option<String>,
    pub country: Option<String>,
    pub label_code: Option<i64>,
    pub annotation: Option<String>,

    pub full_update_date: Option<i64>,
}

impl_get_and_fetch!(Label);
impl_relations!(Label);
impl_db_relation_methods!(Label);
impl_db_relation_fetch_methods!(Label, MBLabel);

impl HasTable for Label {
    const TABLE_NAME: &str = "labels";
    const FOREIGN_FIELD_NAME: &str = "label";
}

impl HasRowID for Label {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTags for Label {}
impl HasGenres for Label {}
impl MBIDRedirection for Label {}

impl HasMBID for Label {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }
}
