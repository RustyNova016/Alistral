use sqlx::SqliteConnection;

use super::genre::create_genre_score_tables;
use super::gid_redirect_tables::generate_redirect_table;
use super::tag::create_tag_tables;

pub(super) async fn create_artist_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
    `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT UNIQUE NOT NULL ,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `country` TEXT,
        `annotation` TEXT,
        
        `full_update_date` INTEGER
    ) STRICT;
     
    CREATE TABLE IF NOT EXISTS `artist_credits_item` (
        `artist_credit` INTEGER REFERENCES `artist_credits` (`id`) ON DELETE CASCADE,
        `position` INTEGER NOT NULL,
        `name` TEXT NOT NULL,
        `artist_gid` TEXT NOT NULL REFERENCES `artists_gid_redirect` (`gid`),
        `join_phrase` TEXT NOT NULL,

        PRIMARY KEY (`artist_credit`, `position`)
    ) STRICT;

    CREATE TABLE IF NOT EXISTS `artist_credits` (`id` INTEGER PRIMARY KEY AUTOINCREMENT) STRICT;

    CREATE TRIGGER `trigger_after_delete_artist_credits` AFTER DELETE ON `artist_credits` BEGIN
        -- If an artist credit is deleted, then unset the integrity flag
        UPDATE `recordings` SET full_update_date = NULL WHERE recordings.artist_credit = OLD.id;
        UPDATE `release_groups` SET full_update_date = NULL WHERE release_groups.artist_credit = OLD.id;
        UPDATE `releases` SET full_update_date = NULL WHERE releases.artist_credit = OLD.id;
    END;

"#,
    )
    .execute(&mut *conn)
    .await?;

    sqlx::query(&generate_redirect_table("artists"))
        .execute(&mut *conn)
        .await?;

    create_tag_tables(conn, "artist", "artists").await?;
    create_genre_score_tables(conn, "artist", "artists").await?;

    Ok(())
}
