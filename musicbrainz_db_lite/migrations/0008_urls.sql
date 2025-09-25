-- Add migration script here
CREATE TABLE `urls` (`id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, `mbid` TEXT UNIQUE NOT NULL, `ressource` TEXT UNIQUE NOT NULL) STRICT;
CREATE TABLE `urls_gid_redirect` (
    `gid` TEXT PRIMARY KEY NOT NULL, 
    `new_id` TEXT REFERENCES `urls`(`id`) ON UPDATE CASCADE ON DELETE SET NULL,
    `deleted` INTEGER DEFAULT 0 NOT NULL
) STRICT;

CREATE TABLE `l_artists_urls` (
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
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

CREATE TABLE `l_genres_urls` (
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
        `entity0` INTEGER NOT NULL REFERENCES `genres` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_labels_urls` (
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
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_recordings_urls` (
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
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_releases_urls` (
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
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
    CREATE TABLE `l_release_groups_urls` (
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
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

    CREATE TABLE `l_urls_urls` (
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
        `entity0` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;
CREATE TABLE `l_urls_works` (
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
        `entity0` INTEGER NOT NULL REFERENCES `urls` (`id`) ON UPDATE CASCADE ON DELETE CASCADE,
        `entity1` INTEGER NOT NULL REFERENCES `works` (`id`) ON UPDATE CASCADE ON DELETE CASCADE
    ) STRICT;

CREATE TRIGGER `trigger_after_insert_urls` AFTER INSERT ON `urls` FOR EACH ROW BEGIN
    INSERT INTO urls_gid_redirect VALUES (new.mbid, new.id, 0) ON CONFLICT DO UPDATE SET new_id = new.id;
END;