use chrono::{DateTime, TimeZone, Utc};
use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;

use crate::models::shared_traits::RowId;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

pub mod deletes;
pub mod relations;
pub mod selects;

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow, Deserialize, Serialize)]
pub struct Listen {
    pub id: i64,

    pub listened_at: i64,

    pub user: String,

    pub recording_msid: String,

    pub data: Option<String>,
}

impl_db_relation_methods!(Listen);

impl Listen {
    pub fn listened_at_as_datetime(&self) -> DateTime<Utc> {
        // unwrap() is best combined with time zone types where the mapping can never fail like Utc.
        Utc.timestamp_opt(self.listened_at, 0).unwrap()
    }

    pub async fn upsert_listen(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO listens VALUES (NULL, ?, ?, ?, ?)",
            self.listened_at,
            self.user,
            self.recording_msid,
            self.data
        )
        .execute(&mut *conn)
        .await?;

        Ok(())
    }
}

impl RowId for Listen {
    fn get_row_id(&self) -> i64 {
        self.id
    }
}

impl HasRowID for Listen {
    fn rowid(&self) -> i64 {
        self.id
    }
}

impl HasTable for Listen {
    const TABLE_NAME: &str = "listens";
    const FOREIGN_FIELD_NAME: &str = "listen";
}
