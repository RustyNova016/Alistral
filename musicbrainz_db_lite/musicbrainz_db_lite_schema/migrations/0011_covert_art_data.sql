
-- Add migration script here
PRAGMA foreign_keys = OFF;

DROP TRIGGER `trigger_after_delete_artist_credits`;
DROP TRIGGER `trigger_after_delete_tracks`;
DROP TRIGGER `trigger_after_delete_release_groups`;

CREATE TABLE `releases_tmp` (
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

                -- Cover art archive
                `artwork` INTEGER,
                `cover_back` INTEGER,
                `cover_count` INTEGER,
                `cover_darkened` INTEGER,
                `cover_front` INTEGER,

                -- Cache management
                `full_update_date` INTEGER CHECK(`full_update_date` > 0),

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits`(`id`) ON DELETE SET NULL,
                `release_group` INTEGER REFERENCES `release_groups` (`id`)
            ) STRICT;
INSERT INTO `releases_tmp` (`id`, `mbid`, `title`, `date`, `country`, `quality`, `status`, `barcode`, `disambiguation`, `packaging`, `annotation`, `full_update_date`, `artist_credit`, `release_group`) SELECT `id`, `mbid`, `title`, `date`, `country`, `quality`, `status`, `barcode`, `disambiguation`, `packaging`, `annotation`, `full_update_date`, `artist_credit`, `release_group` FROM `releases`;
DROP TABLE `releases`;
ALTER TABLE `releases_tmp` RENAME TO `releases`;

-- Restore triggers

CREATE TRIGGER `trigger_after_delete_releases` AFTER DELETE ON `releases` BEGIN
            -- Clean full update date
            UPDATE `release_groups` SET `full_update_date` = NULL WHERE id = OLD.`release_group`;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END;

CREATE TRIGGER `trigger_after_insert_releases` AFTER INSERT ON `releases` FOR EACH ROW BEGIN
    INSERT INTO releases_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;

CREATE TRIGGER `trigger_after_delete_releases_artist_credits` AFTER DELETE ON `releases` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;

CREATE TRIGGER `trigger_after_update_releases_artist_credit` AFTER UPDATE OF `artist_credit` ON `releases` WHEN NEW.artist_credit != OLD.artist_credit BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
END;

CREATE TRIGGER `trigger_after_delete_artist_credits` AFTER DELETE ON `artist_credits` BEGIN
    -- If an artist credit is deleted, then unset the integrity flag
    UPDATE `recordings` SET full_update_date = NULL WHERE recordings.artist_credit = OLD.id;
    UPDATE `release_groups` SET full_update_date = NULL WHERE release_groups.artist_credit = OLD.id;
    UPDATE `releases` SET full_update_date = NULL WHERE releases.artist_credit = OLD.id;
END;

CREATE TRIGGER `trigger_after_delete_tracks` AFTER DELETE ON `tracks` BEGIN
            -- Invalidate the recording as it doesn't have its tracks anymore
            UPDATE `recordings` SET `full_update_date` = NULL WHERE id = OLD.recording;
            UPDATE `releases` SET `full_update_date` = NULL WHERE id = (
                SELECT releases.id 
                FROM releases
                INNER JOIN medias ON releases.id = medias.`release`
                WHERE medias.id = OLD.media
            );
        END;

CREATE TRIGGER `trigger_after_delete_release_groups` AFTER DELETE ON `release_groups` BEGIN
            -- Clean full update date
            UPDATE `releases` SET `full_update_date` = NULL WHERE `release_group` = OLD.id;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END;

PRAGMA foreign_keys = ON;
