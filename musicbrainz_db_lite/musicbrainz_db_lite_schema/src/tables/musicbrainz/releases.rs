use sqlx::SqliteConnection;

use crate::tables::triggers::after_delete_table_artist_credits::after_update_delete_table_artist_credits;

use super::genre::create_genre_score_tables;
use super::gid_redirect_tables::generate_redirect_table;
use super::tag::create_tag_tables;

pub(super) async fn create_release_tables(conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS
            `releases` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `mbid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `date` INTEGER,
                `country` TEXT,
                `quality` TEXT,
                `status` TEXT,
                `barcode` TEXT,
                `disambiguation` TEXT,
                `packaging` TEXT,
                `annotation` TEXT,

                `full_update_date` INTEGER CHECK(`full_update_date` > 0),

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits`(`id`) ON DELETE SET NULL,
                `release_group` INTEGER REFERENCES `release_groups` (`id`)
            ) STRICT;

        CREATE TRIGGER `trigger_after_delete_releases` AFTER DELETE ON `releases` BEGIN
            -- Clean full update date
            UPDATE `release_groups` SET `full_update_date` = NULL WHERE id = OLD.`release_group`;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END;

        CREATE TABLE IF NOT EXISTS
            `medias` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `title` TEXT,
                `position` INTEGER,
                `track_count` INTEGER NOT NULL,
                `disc_count` INTEGER,
                `format` TEXT, 
                `track_offset` INTEGER,

                -- Foreign Keys
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;

        CREATE UNIQUE INDEX IF NOT EXISTS `idx_medias` ON `medias` (`release`, `position`);

        CREATE TABLE IF NOT EXISTS
            `tracks` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `gid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `number` TEXT NOT NULL,
                `length` INTEGER,
                `position` INTEGER NOT NULL,

                -- Foreign Keys
                `media` INTEGER NOT NULL REFERENCES `medias` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `recording` INTEGER REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `artist_credit` INTEGER REFERENCES `artist_credits` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
             
        CREATE UNIQUE INDEX IF NOT EXISTS `idx_tracks` ON `tracks` (`media`, `position`);
        CREATE INDEX `idx_tracks_2` ON `tracks` (`artist_credit`);
        CREATE INDEX `idx_tracks_3` ON `tracks` (`recording`);
        CREATE INDEX `idx_tracks_4` ON `tracks` (`media`);

        CREATE TRIGGER IF NOT EXISTS `trigger_after_delete_tracks` AFTER DELETE ON `tracks` BEGIN
            -- Invalidate the recording as it doesn't have its tracks anymore
            UPDATE `recordings` SET `full_update_date` = NULL WHERE id = OLD.recording;
            UPDATE `releases` SET `full_update_date` = NULL WHERE id = (
                SELECT releases.id 
                FROM releases
                INNER JOIN medias ON releases.id = medias.`release`
                WHERE medias.id = OLD.media
            );
        END;

        CREATE TABLE IF NOT EXISTS
            "label_infos" (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `catalog_number` TEXT,
                `label` TEXT REFERENCES `labels_gid_redirect` (`gid`),
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;

        CREATE INDEX IF NOT EXISTS `idx_label_infos_2` ON `label_infos` (`catalog_number`, `release`);

        CREATE INDEX IF NOT EXISTS `idx_label_infos` ON `label_infos` (`label`, `catalog_number`);

"#
    )
    .execute(&mut *conn)
    .await.unwrap();

    sqlx::query(&generate_redirect_table("releases"))
        .execute(&mut *conn)
        .await
        .unwrap();

    after_update_delete_table_artist_credits(&mut *conn, "releases").await?;

    after_update_delete_table_artist_credits(conn, "tracks").await?;

    create_tag_tables(conn, "release", "releases").await?;
    create_genre_score_tables(conn, "release", "releases").await?;

    Ok(())
}
