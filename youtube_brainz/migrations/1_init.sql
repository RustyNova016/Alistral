-- Add migration script here
CREATE TABLE IF NOT EXISTS "recording" (`id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, `title` TEXT NOT NULL, `artist_credit` TEXT NOT NULL, `release` TEXT, `mbid` TEXT UNIQUE) STRICT;
CREATE TABLE `external_id` (`id` INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, `recording_id` INTEGER NOT NULL REFERENCES `recording`(`id`), `ext_id` TEXT NOT NULL, `service` TEXT NOT NULL, `user_overwrite` TEXT) STRICT;
DELETE FROM sqlite_sequence;
INSERT INTO sqlite_sequence VALUES('recording',0);
CREATE UNIQUE INDEX `idx_recording` ON `recording` (`title`, `artist_credit`, `release`);
CREATE UNIQUE INDEX `idx_external_id` ON `external_id` (`recording_id`, `ext_id`, `service`, `user_overwrite`);