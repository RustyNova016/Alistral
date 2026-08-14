CREATE INDEX IF NOT EXISTS `artist_id` ON artists_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `label_id` ON labels_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `recording_id` ON recordings_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `release_groups_id` ON release_groups_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `releases_id` ON releases_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `urls_id` ON urls_gid_redirect (new_id);
CREATE INDEX IF NOT EXISTS `works_id` ON works_gid_redirect (new_id);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_artists (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_artists (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_genres (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_genres (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_labels (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_labels (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_recordings (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_recordings (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_releases (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_releases (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_artists_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_artists_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_genres (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_genres (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_labels (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_labels (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_recordings (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_recordings (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_releases (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_releases (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_genres_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_genres_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_labels (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_labels (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_recordings (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_recordings (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_releases (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_releases (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_labels_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_labels_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_recordings_recordings (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_recordings_recordings (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_recordings_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_recordings_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_recordings_releases (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_recordings_releases (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_recordings_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_recordings_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_recordings_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_recordings_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_releases_releases (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_releases_releases (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_releases_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_releases_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_releases_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_releases_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_releases_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_releases_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_release_groups_release_groups (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_release_groups_release_groups (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_release_groups_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_release_groups_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_release_groups_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_release_groups_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_urls_urls (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_urls_urls (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_urls_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_urls_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_works_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_works_works (entity1);

CREATE INDEX IF NOT EXISTS `entity0_id` ON l_works_works (entity0);
CREATE INDEX IF NOT EXISTS `entity1_id` ON l_works_works (entity1);

CREATE INDEX IF NOT EXISTS `artist_id` ON artists_genre (artist);
CREATE INDEX IF NOT EXISTS `genre_id` ON artists_genre (genre);

CREATE INDEX IF NOT EXISTS `label_id` ON labels_genre (`label`);
CREATE INDEX IF NOT EXISTS `genre_id` ON labels_genre (genre);

CREATE INDEX IF NOT EXISTS `recording_id` ON recordings_genre (recording);
CREATE INDEX IF NOT EXISTS `genre_id` ON recordings_genre (genre);

CREATE INDEX IF NOT EXISTS `release_group_id` ON release_groups_genre (release_group);
CREATE INDEX IF NOT EXISTS `genre_id` ON release_groups_genre (genre);

CREATE INDEX IF NOT EXISTS `release_id` ON releases_genre (release);
CREATE INDEX IF NOT EXISTS `genre_id` ON releases_genre (genre);

CREATE INDEX IF NOT EXISTS `work_id` ON works_genre (work);
CREATE INDEX IF NOT EXISTS `genre_id` ON works_genre (genre);

CREATE UNIQUE INDEX IF NOT EXISTS `artist_credits_id` ON artist_credits (id);
CREATE INDEX IF NOT EXISTS `artist_credits_id` ON artist_credits_item (artist_credit);

CREATE INDEX IF NOT EXISTS `artist_credit` ON recordings (id);
CREATE INDEX IF NOT EXISTS `artist_credit` ON releases (id);
CREATE INDEX IF NOT EXISTS `artist_credit` ON release_groups (id);