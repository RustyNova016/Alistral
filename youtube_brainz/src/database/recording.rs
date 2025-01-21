use crate::models::messy_recording::MessyRecording;

impl MessyRecording {
    pub async fn upsert(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<MessyRecording, sqlx::Error> {
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
    `artist_credit` = excluded.`artist_credit`,
    `release` = excluded.`release`,
    `mbid` = excluded.`mbid` RETURNING *;",
        )
        .bind(self.title)
        .bind(self.artist_credits)
        .bind(self.release)
        .bind(self.mbid)
        .fetch_one(conn)
        .await
    }
}
