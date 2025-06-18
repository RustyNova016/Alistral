pub(super) async fn create_isrc_table(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE `isrcs` (
            `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, 
            `isrc` TEXT NOT NULL, 
            `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
        ) STRICT;
        CREATE UNIQUE INDEX `unique_recording_isrc` ON `isrcs` (`isrc`, `recording`);
    "#,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
