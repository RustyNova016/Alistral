use musicbrainz_db_lite_macros::Upsert;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

use crate::HasMBID;
use crate::RowId;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_table::HasTable;

pub mod methods;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Upsert, Deserialize, Serialize)]
#[database(
    table = "urls",
    primary_key = "id",
    ignore_insert_keys(id),
    ignore_update_keys(id, mbid)
)]
pub struct Url {
    pub id: i64,
    pub mbid: String,
    pub ressource: String,
}

// === Macros implementations ===

impl_relations!(Url);

// === Trait implementations ===

impl crate::MBIDRedirection for Url {}

impl RowId for Url {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasTable for Url {
    const TABLE_NAME: &str = "urls";
    const FOREIGN_FIELD_NAME: &str = "url";
}

impl HasMBID for Url {
    fn get_mbid(&self) -> &str {
        &self.mbid
    }
}
