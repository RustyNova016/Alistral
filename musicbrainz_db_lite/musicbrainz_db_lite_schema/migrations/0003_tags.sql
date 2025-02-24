-- Add migration script here

CREATE TABLE `artists_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `artist` INTEGER NOT NULL REFERENCES `artists`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_artist` ON `artists_tag` (`name`, `artist`);
CREATE TABLE `labels_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `label` INTEGER NOT NULL REFERENCES `labels`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_label` ON `labels_tag` (`name`, `label`);
CREATE TABLE `recordings_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `recording` INTEGER NOT NULL REFERENCES `recordings`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_recording` ON `recordings_tag` (`name`, `recording`);
CREATE TABLE `release_groups_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release_group` INTEGER NOT NULL REFERENCES `release_groups`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_release_group` ON `release_groups_tag` (`name`, `release_group`);
CREATE TABLE `releases_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `release` INTEGER NOT NULL REFERENCES `releases`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_release` ON `releases_tag` (`name`, `release`);
CREATE TABLE `works_tag` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
                `name` TEXT NOT NULL,
                `count` INTEGER,
                `score` INTEGER,

                -- Foreign keys
                `work` INTEGER NOT NULL REFERENCES `works`(`id`) ON UPDATE CASCADE ON DELETE CASCADE
            ) STRICT;
CREATE UNIQUE INDEX `unique_tag_for_work` ON `works_tag` (`name`, `work`);