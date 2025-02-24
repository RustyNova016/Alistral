PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE `artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        `mbid` TEXT UNIQUE NOT NULL ,
        `name` TEXT NOT NULL,
        `sort_name` TEXT NOT NULL,
        `disambiguation` TEXT NOT NULL,
        `country` TEXT,
        `annotation` TEXT,
        
        `full_update_date` INTEGER
    ) STRICT;
CREATE TABLE `artist_credits_item` (
        `artist_credit` INTEGER REFERENCES `artist_credits` (`id`) ON DELETE CASCADE,
        `position` INTEGER NOT NULL,
        `name` TEXT NOT NULL,
        `artist_gid` TEXT NOT NULL REFERENCES `artists_gid_redirect` (`gid`),
        `join_phrase` TEXT NOT NULL,

        PRIMARY KEY (`artist_credit`, `position`)
    ) STRICT;
CREATE TABLE `artist_credits` (`id` INTEGER PRIMARY KEY AUTOINCREMENT) STRICT;
CREATE TABLE `artists_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `artists_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `artist` INTEGER NOT NULL REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `artists_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `artist` INTEGER NOT NULL REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `genres` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT NOT NULL UNIQUE,
                `name` TEXT NOT NULL,
                `disambiguation` TEXT
            ) STRICT;
CREATE TABLE `recordings` (
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
CREATE TABLE `recordings_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `recordings_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `recordings_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `releases` (
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
CREATE TABLE `medias` (
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
CREATE TABLE `tracks` (
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
CREATE TABLE IF NOT EXISTS "label_infos" (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `catalog_number` TEXT,
                `label` TEXT REFERENCES `labels_gid_redirect` (`gid`),
                `release` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `releases_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `releases_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release` INTEGER NOT NULL REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `releases_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `release` INTEGER NOT NULL REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `release_groups` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `disambiguation` TEXT NOT NULL,
                `primary_type_id` TEXT,
                `first_release_date` INTEGER,
                `annotation` TEXT,

                -- Foreign Keys
                `artist_credit` INTEGER REFERENCES `artist_credits`(`id`) ON DELETE SET NULL,
                
                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;
CREATE TABLE `release_groups_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `release_groups_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release_group` INTEGER NOT NULL REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `release_groups_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `release_group` INTEGER NOT NULL REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `labels` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `label_type` TEXT,
                `sort_name` TEXT,
                `disambiguation` TEXT,
                `country` TEXT,
                `label_code` INTEGER,
                `annotation` TEXT,

                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;
CREATE TABLE `labels_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `labels_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `label` INTEGER NOT NULL REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `labels_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `label` INTEGER NOT NULL REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `works` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT UNIQUE NOT NULL,
                `title` TEXT NOT NULL,
                `work_type` TEXT,
                `disambiguation` TEXT,
                `annotation` TEXT,

                -- Database Utils
                `full_update_date` INTEGER CHECK(`full_update_date` > 0)
            ) STRICT;
CREATE TABLE `works_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;
CREATE TABLE `works_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `work` INTEGER NOT NULL REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `works_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `work` INTEGER NOT NULL REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE TABLE `l_artists_artists` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_artists_labels` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_artists_recordings` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_artists_releases` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_artists_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_artists_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `artists` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_labels_labels` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_labels_recordings` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_labels_releases` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_labels_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_labels_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `labels` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_recordings_recordings` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_recordings_releases` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_recordings_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_recordings_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `recordings` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_releases_releases` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_releases_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_releases_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `releases` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_release_groups_release_groups` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_release_groups_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `release_groups` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_works_works` (
        `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
        `type_id` TEXT NOT NULL,
        `relation_type` TEXT NOT NULL,
        `direction` TEXT NOT NULL, 
        `begin` INTEGER,
        `end` INTEGER,
        `attributes` TEXT,
        `attribute_ids` TEXT,
        `atribute_values` TEXT,
        `target_type` TEXT,
        `target_credit` TEXT,
        `source_credit` TEXT,

        -- Foreign Keys
        `entity0` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE IF NOT EXISTS "users" (
    `id` INTEGER PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL
) STRICT;
CREATE TABLE `messybrainz_submission` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `msid` TEXT UNIQUE NOT NULL,
    `recording` TEXT NOT NULL,
    `artist_credit` TEXT NOT NULL,
    `release` TEXT,
    `track_number` TEXT,
    `duration` INTEGER
) STRICT;
CREATE TABLE IF NOT EXISTS "msid_mapping" (
            `id` INTEGER PRIMARY KEY NOT NULL,

            -- Foreign keys
            `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`) ON DELETE CASCADE,
            `recording_mbid` TEXT NOT NULL REFERENCES `recordings_gid_redirect`(`gid`),
            `user` INTEGER NOT NULL REFERENCES `users`(`id`) ON DELETE CASCADE,
            `release_mbid` TEXT REFERENCES `releases_gid_redirect`(gid)
        ) STRICT;
CREATE TABLE IF NOT EXISTS "listens" (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `listened_at` INTEGER NOT NULL,
    `user` TEXT NOT NULL REFERENCES `users`(`name`) ON DELETE CASCADE,
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`) ON DELETE CASCADE,
    `data` TEXT

    
) STRICT;
DELETE FROM sqlite_sequence;
CREATE TRIGGER `trigger_after_delete_artist_credits` AFTER DELETE ON `artist_credits` BEGIN
        -- If an artist credit is deleted, then unset the integrity flag
        UPDATE `recordings` SET full_update_date = NULL WHERE recordings.artist_credit = OLD.id;
        UPDATE `release_groups` SET full_update_date = NULL WHERE release_groups.artist_credit = OLD.id;
        UPDATE `releases` SET full_update_date = NULL WHERE releases.artist_credit = OLD.id;
    END
;
CREATE TRIGGER `trigger_after_insert_artists` AFTER INSERT ON `artists` FOR EACH ROW BEGIN
    INSERT INTO artists_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE UNIQUE INDEX `unique_tag_for_artist` ON `artists_tag` (`name`, `artist`);
CREATE UNIQUE INDEX `unique_genre_for_artist` ON `artists_genre` (`genre`, `artist`);
CREATE TRIGGER `trigger_after_insert_recordings` AFTER INSERT ON `recordings` FOR EACH ROW BEGIN
    INSERT INTO recordings_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE TRIGGER `trigger_after_delete_recordings_artist_credits` AFTER DELETE ON `recordings` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE TRIGGER `trigger_after_update_recordings_artist_credit` AFTER UPDATE OF `artist_credit` ON `recordings` 
    WHEN NEW.artist_credit != OLD.artist_credit
    BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE UNIQUE INDEX `unique_tag_for_recording` ON `recordings_tag` (`name`, `recording`);
CREATE UNIQUE INDEX `unique_genre_for_recording` ON `recordings_genre` (`genre`, `recording`);
CREATE TRIGGER `trigger_after_delete_releases` AFTER DELETE ON `releases` BEGIN
            -- Clean full update date
            UPDATE `release_groups` SET `full_update_date` = NULL WHERE id = OLD.`release_group`;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END
;
CREATE UNIQUE INDEX `idx_medias` ON `medias` (`release`, `position`);
CREATE UNIQUE INDEX `idx_tracks` ON `tracks` (`media`, `position`);
CREATE INDEX `idx_tracks_2` ON `tracks` (`artist_credit`);
CREATE INDEX `idx_tracks_3` ON `tracks` (`recording`);
CREATE INDEX `idx_tracks_4` ON `tracks` (`media`);
CREATE TRIGGER `trigger_after_delete_tracks` AFTER DELETE ON `tracks` BEGIN
            -- Invalidate the recording as it doesn't have its tracks anymore
            UPDATE `recordings` SET `full_update_date` = NULL WHERE id = OLD.recording;
            UPDATE `releases` SET `full_update_date` = NULL WHERE id = (
                SELECT releases.id 
                FROM releases
                INNER JOIN medias ON releases.id = medias.`release`
                WHERE medias.id = OLD.media
            );
        END
;
CREATE INDEX `idx_label_infos_2` ON `label_infos` (`catalog_number`, `release`);
CREATE INDEX `idx_label_infos` ON `label_infos` (`label`, `catalog_number`);
CREATE TRIGGER `trigger_after_insert_releases` AFTER INSERT ON `releases` FOR EACH ROW BEGIN
    INSERT INTO releases_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE TRIGGER `trigger_after_delete_releases_artist_credits` AFTER DELETE ON `releases` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE TRIGGER `trigger_after_update_releases_artist_credit` AFTER UPDATE OF `artist_credit` ON `releases` 
    WHEN NEW.artist_credit != OLD.artist_credit
    BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE TRIGGER `trigger_after_delete_tracks_artist_credits` AFTER DELETE ON `tracks` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE TRIGGER `trigger_after_update_tracks_artist_credit` AFTER UPDATE OF `artist_credit` ON `tracks` 
    WHEN NEW.artist_credit != OLD.artist_credit
    BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE UNIQUE INDEX `unique_tag_for_release` ON `releases_tag` (`name`, `release`);
CREATE UNIQUE INDEX `unique_genre_for_release` ON `releases_genre` (`genre`, `release`);
CREATE TRIGGER `trigger_after_delete_release_groups` AFTER DELETE ON `release_groups` BEGIN
            -- Clean full update date
            UPDATE `releases` SET `full_update_date` = NULL WHERE `release_group` = OLD.id;

            -- Remove the artist credit
            DELETE FROM `artist_credits` WHERE id = OLD.artist_credit;
        END
;
CREATE TRIGGER `trigger_after_insert_release_groups` AFTER INSERT ON `release_groups` FOR EACH ROW BEGIN
    INSERT INTO release_groups_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE TRIGGER `trigger_after_delete_release_groups_artist_credits` AFTER DELETE ON `release_groups` BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE TRIGGER `trigger_after_update_release_groups_artist_credit` AFTER UPDATE OF `artist_credit` ON `release_groups` 
    WHEN NEW.artist_credit != OLD.artist_credit
    BEGIN
        DELETE FROM artist_credits WHERE artist_credits.id = OLD.artist_credit;
    END;
CREATE UNIQUE INDEX `unique_tag_for_release_group` ON `release_groups_tag` (`name`, `release_group`);
CREATE UNIQUE INDEX `unique_genre_for_release_group` ON `release_groups_genre` (`genre`, `release_group`);
CREATE TRIGGER `trigger_after_insert_labels` AFTER INSERT ON `labels` FOR EACH ROW BEGIN
    INSERT INTO labels_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE UNIQUE INDEX `unique_tag_for_label` ON `labels_tag` (`name`, `label`);
CREATE UNIQUE INDEX `unique_genre_for_label` ON `labels_genre` (`genre`, `label`);
CREATE TRIGGER `trigger_after_insert_works` AFTER INSERT ON `works` FOR EACH ROW BEGIN
    INSERT INTO works_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;
CREATE UNIQUE INDEX `unique_tag_for_work` ON `works_tag` (`name`, `work`);
CREATE UNIQUE INDEX `unique_genre_for_work` ON `works_genre` (`genre`, `work`);
CREATE UNIQUE INDEX `msid_mapping_unique_mapping` ON `msid_mapping` (`recording_msid`, `user`);
CREATE UNIQUE INDEX `idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
COMMIT;
