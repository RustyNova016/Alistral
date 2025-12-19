use chrono::{DateTime, TimeZone, Utc};
use sequelles::has_rowid::HasRowID;
use serde::Deserialize;
use serde::Serialize;
use snafu::ResultExt;

use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::shared_traits::has_table::HasTable;
use crate::utils::macros::hardlink_methods::impl_db_relation_methods;

pub mod deletes;
pub mod fetching;
pub mod insert;
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

    /// Insert or update the listen. If the
    pub async fn upsert_listen(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Listen, SqlxError> {
        sqlx::query_as("INSERT INTO listens VALUES (NULL, $1, $2, $3, $4) ON CONFLICT DO UPDATE SET `listened_at` = excluded.`listened_at`, `user` = excluded.`user`, `recording_msid` = excluded.`recording_msid`, `data` = excluded.`data` RETURNING *")
            .bind(self.listened_at)
            .bind(&self.user)
            .bind(&self.recording_msid)
            .bind(&self.data)
            .fetch_one(&mut *conn)
            .await
            .context(SqlxSnafu)
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

#[cfg(test)]
mod test {
    use crate::User;
    use crate::models::listenbrainz::listen::Listen;
    use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
    use crate::tests::fixtures::default_client::test_mb_client;

    #[tokio::test]
    async fn listen_upsert_test() {
        let client = test_mb_client();
        let conn = &mut *client.get_raw_connection().await.unwrap();

        User::insert_or_ignore(conn, "TestNova").await.unwrap();

        MessybrainzSubmission::builder()
            .id(0)
            .msid("test")
            .recording("Test")
            .artist_credit("Test")
            .build()
            .insert_or_ignore(conn)
            .await
            .unwrap();

        let base_listen = Listen {
            data: None,
            id: 0,
            listened_at: 100,
            recording_msid: "test".to_string(),
            user: "TestNova".to_string(),
        };

        let db_listen = base_listen.upsert_listen(conn).await.unwrap();
        let reinsert = base_listen.upsert_listen(conn).await.unwrap();

        assert_eq!(db_listen, reinsert)
    }
}


