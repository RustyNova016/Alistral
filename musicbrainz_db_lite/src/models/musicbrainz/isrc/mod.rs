use sqlx::FromRow;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
pub struct ISRC {
    id: i64,
    isrc: String,
    recording: i64,
}

impl ISRC {
    pub async fn upsert(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let returned: ISRC = sqlx::query_as(
            "
        INSERT INTO
            `isrcs` (
                `isrc`,
                `recording`
            )
        VALUES
            (?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `isrc` = excluded.`isrc`,
            `recording` = excluded.`recording`
        RETURNING *;",
        )
        .bind(&self.isrc)
        .bind(self.recording)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }

    /// Update the list of ISRCs for a recording. The full list of ISRCs must be given at once.
    pub async fn upsert_recording_isrcs_list(
        conn: &mut sqlx::SqliteConnection,
        recording_id: i64,
        isrcs: Vec<String>,
    ) -> Result<Vec<ISRC>, crate::Error> {
        sqlx::query("DELETE FROM `isrcs` WHERE `recording` = ?")
            .bind(recording_id)
            .execute(&mut *conn)
            .await?;

        let mut out = Vec::new();
        for isrc in isrcs {
            let mut isrc = Self {
                id: 0,
                isrc,
                recording: recording_id,
            };

            isrc.upsert(&mut *conn).await?;
            out.push(isrc);
        }

        Ok(out)
    }
}
