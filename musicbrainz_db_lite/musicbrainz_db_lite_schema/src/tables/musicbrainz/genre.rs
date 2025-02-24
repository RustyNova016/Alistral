pub(super) async fn create_genre_table(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE
            `genres` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT NOT NULL UNIQUE,
                `name` TEXT NOT NULL,
                `disambiguation` TEXT
            ) STRICT;
"#,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub(super) async fn create_genre_score_tables(
    conn: &mut sqlx::SqliteConnection,
    field_name: &str,
    parent_table_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        &format!(r#"
        CREATE TABLE
            `{parent_table_name}_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `{field_name}` INTEGER NOT NULL REFERENCES `{parent_table_name}`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;

        CREATE UNIQUE INDEX `unique_genre_for_{field_name}` ON `{parent_table_name}_genre` (`genre`, `{field_name}`)
"#),
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
