use core::future::Future;

use chrono::DateTime;
use chrono::Utc;
use sqlx::Acquire;
use sqlx::Sqlite;

use crate::models::listenbrainz::listen::Listen;
use crate::utils::sqlx_utils::entity_relations::JoinRelation;

pub struct LatestRecordingListensView {
    pub user: String,
    pub recordings: Vec<String>,
    pub count: i64,
    pub max_ts: Option<DateTime<Utc>>,
}

impl LatestRecordingListensView {
    pub fn execute<'a, 'c, A>(
        &'a self,
        conn: A,
    ) -> impl Future<Output = Result<Vec<JoinRelation<i64, Listen>>, crate::Error>> + Send + 'a
    where
        A: Acquire<'c, Database = Sqlite> + Send + 'a,
    {
        async {
            let recordings = serde_json::to_string(&self.recordings)?;

            let mut conn = conn.acquire().await?;
            Ok(sqlx::query_as(
                "
SELECT
    recordings.id AS original_id,
    listens_sub.*
FROM
    recordings
    INNER JOIN (
        SELECT
            listens.*,
            recordings.id AS recording_id,
            ROW_NUMBER() OVER ( -- Count the rows returned...
                PARTITION BY
                    recordings.id -- ... Per recording...
                ORDER BY
                    listened_at DESC -- ... Sorted by listened at
            ) AS row_num
        FROM
            users
            INNER JOIN listens ON users.name = listens.user
            INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            INNER JOIN recordings ON recordings.id = recordings_gid_redirect.new_id
        WHERE
            -- Only for this user
            LOWER(listens.user) = LOWER(?)
            -- Keep only mapped listens 
            AND msid_mapping.user = users.id
            -- before date
            AND listens.listened_at <= ?
    ) AS listens_sub ON listens_sub.recording_id = recordings.id
WHERE
    listens_sub.recording_id = recordings.id
    AND row_num <= ?
    AND recordings.mbid IN (
        SELECT value FROM JSON_EACH(?)
    );
                ",
            )
            .bind(&self.user)
            .bind(self.max_ts.unwrap_or(DateTime::<Utc>::MAX_UTC).timestamp())
            .bind(self.count)
            .bind(&recordings)
            .fetch_all(&mut *conn)
            .await?)
        }
    }
}
