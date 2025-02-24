use listens::create_listen_tables;
use msid_mapping::create_msid_mapping_tables;

pub mod listens;
pub mod msid_mapping;

pub async fn generate_listenbrainz_database(
    conn: &mut sqlx::SqliteConnection,
) -> Result<(), sqlx::Error> {
    create_msid_mapping_tables(conn).await?;
    create_listen_tables(conn).await?;

    Ok(())
}
