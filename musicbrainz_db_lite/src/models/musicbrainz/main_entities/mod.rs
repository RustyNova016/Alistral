pub mod convert;
use std::sync::Arc;

use sequelles::has_rowid::HasRowID;
#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsync;

use crate::FetchAndSave;
use crate::Track;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::shared_traits::HasMBID;

use super::artist::Artist;
use super::label::Label;
use super::recording::Recording;
use super::release::Release;
use super::work::Work;

pub mod crawler;

/// Contain any of the main entities
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MainEntity {
    Artist(Artist),
    Label(Label),
    Recording(Recording),
    Release(Release),
    Track(Track),
    Work(Work),
}

impl MainEntity {
    /// Return true if the two enums have the same discriminant and the same MBID
    pub fn is_equal_by_mbid(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
            && self.get_mbid() == other.get_mbid()
    }

    // pub async fn refetch_and_load(
    //     &mut self,
    //     conn: &mut sqlx::SqliteConnection,
    //     client: &crate::DBClient,
    // ) -> Result<(), crate::Error> {
    //     match self {
    //         MainEntity::Artist(val) => val.refetch_and_load(conn, client).await?,
    //         MainEntity::Label(val) => val.refetch_and_load(conn, client).await?,
    //         MainEntity::Recording(val) => val.refetch_and_load(conn, client).await?,
    //         MainEntity::Release(val) => val.refetch_and_load(conn, client).await?,
    //         MainEntity::Track(val) => val.refetch_and_load(conn, client).await?,
    //         MainEntity::Work(val) => val.refetch_and_load(conn, client).await?,
    //     }

    //     Ok(())
    // }

    pub async fn refetch_and_load_as_task(
        &mut self,
        client: Arc<crate::DBClient>,
    ) -> Result<(), crate::Error> {
        match self {
            MainEntity::Artist(val) => {
                *self = MainEntity::Artist(val.refetch_as_task(client).await?);
            }
            MainEntity::Label(val) => {
                *self = MainEntity::Label(val.refetch_as_task(client).await?);
            }
            MainEntity::Recording(val) => {
                *self = MainEntity::Recording(val.refetch_as_task(client).await?);
            }
            MainEntity::Release(val) => {
                *self = MainEntity::Release(val.refetch_as_task(client).await?);
            }
            MainEntity::Track(val) => {
                *self = MainEntity::Track(val.refetch_as_task(client).await?);
            }
            MainEntity::Work(val) => {
                *self = MainEntity::Work(val.refetch_as_task(client).await?);
            }
        }

        Ok(())
    }

    pub fn get_unique_id(&self) -> String {
        match self {
            MainEntity::Artist(val) => format!("artist_{}", val.mbid),
            MainEntity::Label(val) => format!("label_{}", val.mbid),
            MainEntity::Recording(val) => format!("recording_{}", val.mbid),
            MainEntity::Release(val) => format!("release_{}", val.mbid),
            MainEntity::Track(val) => format!("track_{}", val.gid),
            MainEntity::Work(val) => format!("work_{}", val.mbid),
        }
    }

    pub fn get_musicbrainz_link(&self) -> String {
        let path = match self {
            MainEntity::Artist(_) => "artist",
            MainEntity::Label(_) => "label",
            MainEntity::Recording(_) => "recording",
            MainEntity::Release(_) => "release",
            MainEntity::Track(_) => "track",
            MainEntity::Work(_) => "work",
        };

        format!("https://musicbrainz.org/{path}/{}", self.get_mbid())
    }
}

impl HasRowID for MainEntity {
    fn rowid(&self) -> i64 {
        match self {
            Self::Artist(val) => val.rowid(),
            Self::Label(val) => val.rowid(),
            Self::Recording(val) => val.rowid(),
            Self::Release(val) => val.rowid(),
            Self::Track(val) => val.rowid(),
            Self::Work(val) => val.rowid(),
        }
    }
}

impl HasMBID for MainEntity {
    fn get_mbid(&self) -> &str {
        match self {
            Self::Artist(val) => val.get_mbid(),
            Self::Label(val) => val.get_mbid(),
            Self::Recording(val) => val.get_mbid(),
            Self::Release(val) => val.get_mbid(),
            Self::Track(val) => val.get_mbid(),
            Self::Work(val) => val.get_mbid(),
        }
    }
}

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater> for MainEntity {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        let out = match self {
            MainEntity::Artist(val) => val.format_with_async(ft).await?,
            MainEntity::Label(val) => val.format_with_async(ft).await?,
            MainEntity::Recording(val) => val.format_with_async(ft).await?,
            MainEntity::Release(val) => val.format_with_async(ft).await?,
            MainEntity::Track(val) => val.format_with_async(ft).await?,
            MainEntity::Work(val) => val.format_with_async(ft).await?,
        };

        Ok(out)
    }
}
