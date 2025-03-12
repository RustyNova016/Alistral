use core::future::Future;

use chrono::DateTime;
use chrono::Utc;
use sqlx::Acquire;
use sqlx::Sqlite;

use crate::models::listenbrainz::listen::Listen;

pub struct LatestRecordingListensView {
    pub user: String,
    pub recording: String,
    pub max_ts: Option<DateTime<Utc>>,
}

impl LatestRecordingListensView {
    pub fn execute<'a, 'c, A>(
        &'a self,
        conn: A,
    ) -> impl Future<Output = Result<Vec<Listen>, crate::Error>> + Send + 'a
    where
        A: Acquire<'c, Database = Sqlite> + Send + 'a,
    {
        async {
            let mut conn = conn.acquire().await?;
            Ok(sqlx::query_as(
                "
        SELECT
            listens.*
        FROM
            users
            INNER JOIN listens ON users.name = listens.user
            INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
        WHERE
            -- Only for this user
            LOWER(listens.user) = LOWER(?)
            
            -- Keep only mapped listens 
            AND msid_mapping.user = users.id
        
            -- before date
            AND listens.listened_at <= ?
        
            -- Filter recordings
            AND msid_mapping.recording_mbid IN (
                SELECT
                    gid
                FROM
                    recordings_gid_redirect
                WHERE
                    new_id = ?
            )
        
                ",
            )
            .bind(&self.user)
            .bind(self.max_ts.unwrap_or(DateTime::<Utc>::MAX_UTC).timestamp())
            .bind(&self.recording)
            .fetch_all(&mut *conn)
            .await?)
        }
    }
}
