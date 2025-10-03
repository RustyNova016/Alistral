use sqlx::Acquire;

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
    let mut trans = conn.begin().await?;

    cleanup_table_data(&mut trans, "artists").await?;
    cleanup_table_data(&mut trans, "labels").await?;
    cleanup_table_data(&mut trans, "recordings").await?;
    cleanup_table_data(&mut trans, "releases").await?;
    // cleanup_table_data(&mut trans, "release_groups").await?;
    cleanup_table_data(&mut trans, "works").await?;
    
    trans.commit().await?;

    Ok(())
}
