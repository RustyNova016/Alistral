CREATE TABLE `artists_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `artist` INTEGER NOT NULL REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_artist` ON `artists_genre` (`genre`, `artist`);
CREATE TABLE `genres` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `mbid` TEXT NOT NULL UNIQUE,
                `name` TEXT NOT NULL,
                `disambiguation` TEXT
            ) STRICT;
CREATE TABLE `labels_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `label` INTEGER NOT NULL REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_label` ON `labels_genre` (`genre`, `label`);
CREATE TABLE `recordings_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_recording` ON `recordings_genre` (`genre`, `recording`);
CREATE TABLE `release_groups_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `release_group` INTEGER NOT NULL REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_release_group` ON `release_groups_genre` (`genre`, `release_group`);
CREATE TABLE `releases_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `release` INTEGER NOT NULL REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_release` ON `releases_genre` (`genre`, `release`);
CREATE TABLE `works_genre` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `count` INTEGER,

                -- Foreign keys
                `work` INTEGER NOT NULL REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE CASCADE,
                `genre` INTEGER NOT NULL REFERENCES `genres`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_genre_for_work` ON `works_genre` (`genre`, `work`);

UPDATE `artists` SET `full_update_date` = NULL;
UPDATE `labels` SET `full_update_date` = NULL;
UPDATE `recordings` SET `full_update_date` = NULL;
UPDATE `releases` SET `full_update_date` = NULL;
UPDATE `release_groups` SET `full_update_date` = NULL;
UPDATE `works` SET `full_update_date` = NULL;