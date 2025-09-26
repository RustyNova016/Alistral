-- Add migration script here
ALTER TABLE l_artists_artists ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_artists SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_genres ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_genres SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_labels ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_labels SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_recordings ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_recordings SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_releases ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_releases SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_artists_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_artists_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_genres ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_genres SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_labels ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_labels SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_recordings ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_recordings SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_releases ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_releases SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_genres_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_genres_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_labels ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_labels SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_recordings ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_recordings SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_releases ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_releases SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_labels_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_labels_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_recordings_recordings ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_recordings_recordings SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_recordings_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_recordings_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_recordings_releases ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_recordings_releases SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_recordings_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_recordings_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_recordings_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_recordings_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_release_groups_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_release_groups_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_release_groups_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_release_groups_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_release_groups_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_release_groups_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_releases_release_groups ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_releases_release_groups SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_releases_releases ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_releases_releases SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_releases_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_releases_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_releases_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_releases_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_urls_urls ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_urls_urls SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_urls_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_urls_works SET `ended` = 1 WHERE `end` IS NOT NULL;

ALTER TABLE l_works_works ADD COLUMN `ended` INTEGER NOT NULL DEFAULT 0;
UPDATE l_works_works SET `ended` = 1 WHERE `end` IS NOT NULL;