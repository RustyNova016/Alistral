pub(super) async fn create_listen_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
CREATE TABLE IF NOT EXISTS "listens" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `listened_at` INTEGER NOT NULL,
    `user` TEXT NOT NULL REFERENCES `users`(`name`) ON DELETE CASCADE,
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`) ON DELETE CASCADE,
    `data` TEXT

    
) STRICT;

-- Indexes

CREATE UNIQUE INDEX IF NOT EXISTS`idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
"#
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
