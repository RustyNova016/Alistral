use crate::tables::musicbrainz::gid_redirect_tables::generate_redirect_table;

pub(super) async fn create_url_tables(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE `urls` (`id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, `mbid` TEXT UNIQUE NOT NULL, `ressource` TEXT UNIQUE NOT NULL) STRICT
"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("urls"))
        .execute(&mut *conn)
        .await?;

    Ok(())
}
