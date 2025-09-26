use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

use crate::HasMBID;
use crate::models::musicbrainz::relations::impl_relations::impl_relations;
use crate::models::shared_traits::has_table::HasTable;

pub mod methods;
pub mod upsert;

#[derive(Debug, Default, PartialEq, Eq, Clone, FromRow, Deserialize, Serialize)]
pub struct Url {
    pub id: i64,
    pub mbid: String,
    pub ressource: String,
}

// === Macros implementations ===

impl_relations!(Url);

// === Trait implementations ===

impl crate::MBIDRedirection for Url {}

impl HasRowID for Url {
    fn rowid(&self) -> i64 {
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
