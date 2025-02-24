/// Trigger that delete the artist credit attached to the entity when this entity is deleted
pub async fn after_update_delete_table_artist_credits(
    conn: &mut sqlx::SqliteConnection,
    table: &str,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(&format!(
        "
    CREATE TRIGGER `trigger_after_delete_{table}_artist_credits` AFTER DELETE ON `{table}` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;

    CREATE TRIGGER `trigger_after_update_{table}_artist_credit` AFTER UPDATE OF `artist_credit` ON `{table}` 
    WHEN NEW.artist_credit != OLD.artist_credit
    BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
    "
    ))
    .execute(conn)
    .await
}
