use chrono::{DateTime, TimeZone, Utc};
use serde::Deserialize;
use serde::Serialize;

use crate::models::shared_traits::RowId;

pub mod deletes;
pub mod querry_builder;
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
