use sqlx::SqliteConnection;

pub(super) async fn create_relation_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    create_relation_table(conn, "artists", "artists").await?;
    create_relation_table(conn, "artists", "genres").await?;
    create_relation_table(conn, "artists", "labels").await?;
    create_relation_table(conn, "artists", "recordings").await?;
    create_relation_table(conn, "artists", "releases").await?;
    create_relation_table(conn, "artists", "release_groups").await?;
    create_relation_table(conn, "artists", "urls").await?;
    create_relation_table(conn, "artists", "works").await?;

    create_relation_table(conn, "genres", "genres").await?;
    create_relation_table(conn, "genres", "labels").await?;
    create_relation_table(conn, "genres", "recordings").await?;
    create_relation_table(conn, "genres", "releases").await?;
    create_relation_table(conn, "genres", "release_groups").await?;
    create_relation_table(conn, "genres", "urls").await?;
    create_relation_table(conn, "genres", "works").await?;

    create_relation_table(conn, "labels", "labels").await?;
    create_relation_table(conn, "labels", "recordings").await?;
    create_relation_table(conn, "labels", "releases").await?;
    create_relation_table(conn, "labels", "release_groups").await?;
    create_relation_table(conn, "labels", "urls").await?;
    create_relation_table(conn, "labels", "works").await?;

    create_relation_table(conn, "recordings", "recordings").await?;
    create_relation_table(conn, "recordings", "releases").await?;
    create_relation_table(conn, "recordings", "release_groups").await?;
    create_relation_table(conn, "recordings", "urls").await?;
    create_relation_table(conn, "recordings", "works").await?;

    create_relation_table(conn, "releases", "releases").await?;
    create_relation_table(conn, "releases", "release_groups").await?;
    create_relation_table(conn, "releases", "urls").await?;
    create_relation_table(conn, "releases", "works").await?;

    create_relation_table(conn, "release_groups", "release_groups").await?;
    create_relation_table(conn, "release_groups", "urls").await?;
    create_relation_table(conn, "release_groups", "works").await?;

    create_relation_table(conn, "urls", "urls").await?;
    create_relation_table(conn, "urls", "works").await?;

    create_relation_table(conn, "works", "works").await?;

    Ok(())
}

pub(super) async fn create_relation_table(
    conn: &mut SqliteConnection,
    table_a: &str,
    table_b: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(&format!(
        r#"
    CREATE TABLE
    `l_{table_a}_{table_b}` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `ended` INTEGER NOT NULL DEFAULT 0,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `{table_a}` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `{table_b}` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT

        "#
    ))
    .execute(&mut *conn)
    .await?;

    Ok(())
}
