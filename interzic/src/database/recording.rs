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
}
