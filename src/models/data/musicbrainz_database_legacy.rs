use std::mem::discriminant;
use std::sync::Arc;

use color_eyre::Report;
use derive_getters::Getters;
use once_cell::sync::Lazy;
use tokio::try_join;

use crate::core::caching::musicbrainz_cache::MusicbrainzCacheLegacy;
use crate::models::cli::cache::ClearTarget;
use crate::models::data::musicbrainz::artist::mbid::ArtistMBID;
use crate::models::data::musicbrainz::mbid::MBID;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;
use crate::models::data::musicbrainz::release::mbid::ReleaseMBID;
use crate::models::data::musicbrainz::release_group::mbid::ReleaseGroupMBID;
use crate::models::data::musicbrainz::release_group::ReleaseGroup;
use crate::models::data::musicbrainz::work::mbid::WorkMBID;
use crate::models::error::Error;

use super::musicbrainz::artist::Artist;
use super::musicbrainz::recording::Recording;
use super::musicbrainz::release::Release;
use super::musicbrainz::work::Work;

#[deprecated]
pub(crate) static MUSICBRAINZ_DATABASE_LEGACY: Lazy<Arc<MusicBrainzDatabaseLegacy>> =
    Lazy::new(|| Arc::new(MusicBrainzDatabaseLegacy::default()));

#[derive(Debug, Getters)]
#[deprecated]
pub struct MusicBrainzDatabaseLegacy {
    artists: Arc<MusicbrainzCacheLegacy<ArtistMBID, Artist>>,
    releases: Arc<MusicbrainzCacheLegacy<ReleaseMBID, Release>>,
    recordings: Arc<MusicbrainzCacheLegacy<RecordingMBID, Recording>>,
    release_groups: Arc<MusicbrainzCacheLegacy<ReleaseGroupMBID, ReleaseGroup>>,
    works: Arc<MusicbrainzCacheLegacy<WorkMBID, Work>>,
}

impl MusicBrainzDatabaseLegacy {
    pub async fn remove(&self, id: &MBID) -> color_eyre::Result<()> {
        match id {
            MBID::Artist(id) => self.artists.remove(id).await?,
            MBID::Release(id) => self.releases.remove(id).await?,
            MBID::Recording(id) => self.recordings.remove(id).await?,
            MBID::ReleaseGroup(id) => self.release_groups.remove(id).await?,
            MBID::Work(id) => self.works.remove(id).await?,
        }

        Ok(())
    }

    pub async fn invalidate_last_entries(
        &self,
        k: usize,
        keep_min: usize,
    ) -> color_eyre::Result<()> {
        try_join!(
            self.artists.invalidate_last_entries(k, keep_min),
            self.releases.invalidate_last_entries(k, keep_min),
            self.recordings.invalidate_last_entries(k, keep_min),
            self.release_groups.invalidate_last_entries(k, keep_min),
            self.works.invalidate_last_entries(k, keep_min)
        )?;

        Ok(())
    }

    pub async fn add_alias(&self, alias: &MBID, main: &MBID) -> color_eyre::Result<()> {
        // Check if both are the same variant
        if discriminant(alias) != discriminant(main) {
            return Err(Report::from(Error::MBIDAliasError(
                alias.clone(),
                main.clone(),
            )));
        }

        let main = main.clone();

        match alias {
            MBID::Artist(alias) => {
                self.artists
                    .insert_alias(alias, &main.unwrap_artist())
                    .await?;
            }
            MBID::Release(alias) => {
                self.releases
                    .insert_alias(alias, &main.unwrap_release())
                    .await?;
            }
            MBID::Work(alias) => self.works.insert_alias(alias, &main.unwrap_work()).await?,
            MBID::ReleaseGroup(alias) => {
                self.release_groups
                    .insert_alias(alias, &main.unwrap_release_group())
                    .await?;
            }
            MBID::Recording(alias) => {
                self.recordings
                    .insert_alias(alias, &main.unwrap_recording())
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn clear(&self, target: &ClearTarget) -> cacache::Result<()> {
        match target {
            ClearTarget::All => {
                let _ = try_join!(
                    self.artists.clear(),
                    self.releases.clear(),
                    self.recordings.clear(),
                    self.release_groups.clear(),
                    self.works.clear()
                )?;
            }
        }

        Ok(())
    }
}

impl Default for MusicBrainzDatabaseLegacy {
    fn default() -> Self {
        Self {
            artists: Arc::new(MusicbrainzCacheLegacy::new("artists")),
            releases: Arc::new(MusicbrainzCacheLegacy::new("releases")),
            recordings: Arc::new(MusicbrainzCacheLegacy::new("recordings")),
            release_groups: Arc::new(MusicbrainzCacheLegacy::new("release_groups")),
            works: Arc::new(MusicbrainzCacheLegacy::new("works")),
        }
    }
}