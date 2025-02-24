pub mod listenbrainz;
pub mod musicbrainz;
pub mod triggers;
use musicbrainz::generate_musicbrainz_database;
use sqlx::SqliteConnection;

pub async fn create_listenbrainz_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"PRAGMA foreign_keys = OFF; 

-- Tables
CREATE TABLE IF NOT EXISTS "users" (
    `id` INTEGER PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS `messybrainz_submission` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `msid` TEXT UNIQUE NOT NULL,
    `recording` TEXT NOT NULL,
    `artist_credit` TEXT NOT NULL,
    `release` TEXT,
    `track_number` TEXT,
    `duration` INTEGER
) STRICT;

PRAGMA foreign_keys = ON;"#,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn create_musicbrainz_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    generate_musicbrainz_database(conn).await?;
    Ok(())
}
