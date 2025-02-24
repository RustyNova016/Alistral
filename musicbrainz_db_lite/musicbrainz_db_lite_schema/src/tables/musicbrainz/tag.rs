pub(super) async fn create_tag_tables(
    conn: &mut sqlx::SqliteConnection,
    field_name: &str,
    parent_table_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        &format!(r#"
        CREATE TABLE
            `{parent_table_name}_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `{field_name}` INTEGER NOT NULL REFERENCES `{parent_table_name}`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;

        CREATE UNIQUE INDEX `unique_tag_for_{field_name}` ON `{parent_table_name}_tag` (`name`, `{field_name}`)
"#),
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
