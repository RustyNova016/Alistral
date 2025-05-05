use async_fn_stream::try_fn_stream;
use futures::StreamExt;
use futures::TryStreamExt;
use sqlx::Acquire;
use sqlx::Sqlite;

use crate::models::messy_recording::MessyRecording;

impl MessyRecording {
    pub async fn upsert<'a, A>(self, conn: A) -> Result<MessyRecording, crate::Error>
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
        .bind(self.title.clone())
        .bind(self.artist_credits.clone())
        .bind(self.release.clone())
        .bind(self.mbid.clone())
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| crate::Error::MessyRecordingSaveError(err, self))
    }

    /// Return all the recordings that are mapped to this `ext_id`.
    pub async fn recordings_from_mapping<'a, A>(
        conn: A,
        ext_id: &str,
        service: &str,
        user_overwrite: Option<&str>,
    ) -> Result<Vec<MessyRecording>, sqlx::Error>
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        let mut conn = conn.acquire().await?;
        sqlx::query_as(
            "
SELECT recording.* 
FROM recording
    INNER JOIN external_id ON recording.id = external_id.recording_id
WHERE ext_id = $1
    AND service = $2
    AND (
        LOWER(user_overwrite) = LOWER($3)
        OR (
            user_overwrite = ''
            AND recording_id NOT IN (
                -- Remove all the mappings that got overwritten
                SELECT recording_id 
                FROM external_id 
                WHERE service = $2 
                    AND LOWER(user_overwrite) = LOWER($3)
                )
        )
    );",
        )
        .bind(ext_id)
        .bind(service)
        .bind(user_overwrite.unwrap_or_default())
        .fetch_all(&mut *conn)
        .await
    }

    pub fn iter_recordings_with_mbids<'a, A>(
        conn: A,
    ) -> impl TryStreamExt<Ok = MessyRecording, Error = sqlx::Error> + Unpin
    where
        A: Acquire<'a, Database = Sqlite>,
    {
        Box::pin(try_fn_stream(|emitter| async move {
            let mut conn = match conn.acquire().await {
                Err(err) => {
                    emitter.emit_err(err).await;
                    return Ok(());
                }
                Ok(conn) => conn,
            };

            let mut stream =
                sqlx::query_as("SELECT * FROM recording WHERE mbid IS NOT NULL;").fetch(&mut *conn);
            while let Some(val) = stream.next().await {
                match val {
                    Ok(val) => emitter.emit(val).await,
                    Err(err) => emitter.emit_err(err).await,
                };
            }

            Ok(())
        }))
    }
}
