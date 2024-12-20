use sqlx::Acquire;
use sqlx::Sqlite;

use crate::models::external_id::ExternalId;

impl ExternalId {
    pub async fn upsert<'a, A>(self, conn: A) -> Result<ExternalId, sqlx::Error>
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        let mut conn = conn.acquire().await?;

        sqlx::query_as(
            "
INSERT INTO
    external_id
VALUES
    (NULL, ?, ?, ?, ?)
ON CONFLICT DO
UPDATE
SET
    `recording_id` = excluded.`recording_id`,
    `ext_id` = excluded.`ext_id`,
    `service` = excluded.`service`,
    `user_overwrite` = excluded.`user_overwrite` RETURNING *;",
        )
        .bind(self.recording_id)
        .bind(self.ext_id)
        .bind(self.service)
        .bind(self.user_overwrite)
        .fetch_one(&mut *conn)
        .await
    }

    pub async fn find<'a, A>(
        conn: A,
        recording: i64,
        service: String,
        user_overwrite: Option<String>,
    ) -> Result<Option<String>, sqlx::Error>
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        //TODO: Make user overwrite return default result

        let mut conn = conn.acquire().await?;
        sqlx::query_scalar("SELECT ext_id FROM external_id WHERE recording_id = ? AND service = ? AND user_overwrite = ?;")
            .bind(recording)
            .bind(service)
            .bind(user_overwrite.unwrap_or_default())
            .fetch_optional(&mut *conn).await
    }
}
