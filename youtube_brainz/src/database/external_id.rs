use crate::models::external_id::ExternalId;

impl ExternalId {
    pub async fn upsert(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<ExternalId, sqlx::Error> {
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
        .fetch_one(conn)
        .await
    }

    pub async fn find(
        conn: &mut sqlx::SqliteConnection,
        recording: i64,
        service: String,
        user_overwrite: Option<String>,
    ) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar("SELECT ext_id FROM external_id WHERE recording_id = ? AND service = ? AND user_overwrite = ?;")
            .bind(recording)
            .bind(service)
            .bind(user_overwrite)
            .fetch_optional(conn).await
    }
}
