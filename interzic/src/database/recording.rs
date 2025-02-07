use sqlx::Acquire;
use sqlx::Sqlite;

use crate::models::messy_recording::MessyRecording;

impl MessyRecording {
    pub async fn upsert<'a, A>(self, conn: A) -> Result<MessyRecording, sqlx::Error>
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        let mut conn = conn.acquire().await?;
        sqlx::query_as(
            "
            INSERT INTO
                recording
            VALUES
                (NULL, ?, ?, ?, ?)
            ON CONFLICT DO
            UPDATE
            SET
                `title` = excluded.`title`,
                `artist_credits` = excluded.`artist_credits`,
                `release` = excluded.`release`,
                `mbid` = excluded.`mbid` RETURNING *;",
        )
        .bind(self.title)
        .bind(self.artist_credits)
        .bind(self.release)
        .bind(self.mbid)
        .fetch_one(&mut *conn)
        .await
    }

    pub async fn find_from_mapping<'a, A>(
        conn: A,
        ext_id: &str,
        service: &str,
        user_overwrite: Option<&str>,
    ) -> Result<Option<MessyRecording>, sqlx::Error>
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        let mut conn = conn.acquire().await?;
        sqlx::query_as(
            "
            SELECT recording.* 
FROM recording
    INNER JOIN external_id ON recording.id = external_id.recording_id
WHERE ext_id = ?
    AND service = ?
    AND (user_overwrite = ? OR user_overwrite = '')
ORDER BY user_overwrite DESC
LIMIT 1;",
        )
        .bind(ext_id)
        .bind(service)
        .bind(user_overwrite.unwrap_or_default())
        .fetch_optional(&mut *conn)
        .await
    }
}
