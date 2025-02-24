use sqlx::SqliteConnection;

use crate::tables::triggers::after_delete_table_artist_credits::after_update_delete_table_artist_credits;

use super::genre::create_genre_score_tables;
use super::gid_redirect_tables::generate_redirect_table;
use super::tag::create_tag_tables;

pub(super) async fn create_recordings_tables(
    conn: &mut SqliteConnection,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS `recordings` (
            `id` INTEGER PRIMARY KEY NOT NULL, 
            `mbid` TEXT UNIQUE NOT NULL, 
            `title` TEXT NOT NULL, 
            `video` INTEGER CHECK(`video` = 0 OR `video` = 1),
            `length` INTEGER, 
            `disambiguation` TEXT,  
            `annotation` TEXT,
            `first_release_date` INTEGER,

            `full_update_date` INTEGER CHECK(`full_update_date` > 0),

            -- Foreign keys
            `artist_credit` INTEGER REFERENCES `artist_credits`(`id`) ON DELETE SET NULL
        ) STRICT;
"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("recordings"))
        .execute(&mut *conn)
        .await?;

    after_update_delete_table_artist_credits(conn, "recordings").await?;

    create_tag_tables(conn, "recording", "recordings").await?;
    create_genre_score_tables(conn, "recording", "recordings").await?;

    Ok(())
}
