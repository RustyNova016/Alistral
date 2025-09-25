-- Add migration script here

PRAGMA foreign_keys = OFF;
CREATE TABLE `listens_tmp` (
    `id` INTEGER PRIMARY KEY NOT NULL,
    `listened_at` INTEGER NOT NULL,
    `user` TEXT NOT NULL REFERENCES `users`(`name`) ON DELETE CASCADE,
    `recording_msid` TEXT NOT NULL REFERENCES `messybrainz_submission`(`msid`) ON DELETE CASCADE,
    `data` TEXT

    
) STRICT;
INSERT INTO `listens_tmp` (`id`, `listened_at`, `user`, `recording_msid`, `data`) SELECT `id`, `listened_at`, `user`, `recording_msid`, `data` FROM `listens`;
DROP TABLE `listens`;
ALTER TABLE `listens_tmp` RENAME TO `listens`;
CREATE UNIQUE INDEX `idx_listens` ON `listens` (`listened_at`, `user`, `recording_msid`);
PRAGMA foreign_keys = ON;