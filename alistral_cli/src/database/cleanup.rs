pub async fn cleanup_table_data(
    conn: &mut sqlx::SqliteConnection,
    table_name: &str,
) -> Result<(), sqlx::Error> {
    let sql = format!(
        "DELETE FROM {table_name} WHERE {table_name}.id IN (SELECT id FROM {table_name} WHERE full_update_date IS NOT NULL ORDER BY full_update_date LIMIT 10)"
    );
    sqlx::query(&sql).execute(&mut *conn).await?;

    let sql = format!(
        "DELETE FROM {table_name} WHERE {table_name}.id IN (SELECT id FROM {table_name} ORDER BY full_update_date LIMIT 10)"
    );
    sqlx::query(&sql).execute(conn).await?;

    Ok(())
}

pub async fn cleanup_database(conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
    cleanup_table_data(conn, "recordings").await?;
    cleanup_table_data(conn, "artists").await?;
    cleanup_table_data(conn, "releases").await?;
    cleanup_table_data(conn, "labels").await?;

    Ok(())
}
