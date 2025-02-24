pub(super) async fn create_msid_mapping_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DROP TABLE IF EXISTS "msid_mapping";
        CREATE TABLE IF NOT EXISTS "msid_mapping" (
            `id` INTEGER PRIMARY KEY NOT NULL,

            -- Foreign keys
            `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`) ON DELETE CASCADE,
            `recording_mbid` TEXT NOT NULL REFERENCES `recordings_gid_redirect`(`gid`),
            `user` INTEGER NOT NULL REFERENCES `users`(`id`) ON DELETE CASCADE,
            `release_mbid` TEXT REFERENCES `releases_gid_redirect`(gid)
        ) STRICT;

        CREATE UNIQUE INDEX IF NOT EXISTS `msid_mapping_unique_mapping` ON `msid_mapping` (`recording_msid`, `user`);
"#
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
