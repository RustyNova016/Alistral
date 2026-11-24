-- Cannot make it unique due to null rows being duplicated.
-- Speedup deletion triggers
CREATE INDEX IF NOT EXISTS `idx_recordings_artist_credit` ON `recordings` (`artist_credit`);
CREATE INDEX IF NOT EXISTS `idx_releases_artist_credit` ON `releases` (`artist_credit`);
CREATE INDEX IF NOT EXISTS `idx_release_groups_artist_credit` ON `release_groups` (`artist_credit`);
