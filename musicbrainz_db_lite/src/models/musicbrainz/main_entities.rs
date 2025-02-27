use crate::models::shared_traits::HasMBID;
#[cfg(feature = "pretty_format")]
use crate::DBClient;
use crate::RowId;

use super::artist::Artist;
use super::label::Label;
use super::recording::Recording;
use super::release::Release;
use super::work::Work;

/// Contain any of the main entities
#[derive(Debug, PartialEq, Eq)]
pub enum MainEntity {
    Artist(Artist),
    Label(Label),
    Recording(Recording),
    Release(Release),
    Work(Work),
}

impl MainEntity {
    /// Return true if the two enums have the same discriminant and the same MBID
    pub fn is_equal_by_mbid(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
            && self.get_mbid() == other.get_mbid()
    }

    pub async fn refetch_and_load(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<(), crate::Error> {
        match self {
            MainEntity::Artist(val) => val.refetch_and_load(conn, client).await?,
            MainEntity::Label(val) => val.refetch_and_load(conn, client).await?,
            MainEntity::Recording(val) => val.refetch_and_load(conn, client).await?,
            MainEntity::Release(val) => val.refetch_and_load(conn, client).await?,
            MainEntity::Work(val) => val.refetch_and_load(conn, client).await?,
        }

        Ok(())
    }
}

impl RowId for MainEntity {
    fn get_row_id(&self) -> i64 {
        match self {
            Self::Artist(val) => val.get_row_id(),
            Self::Label(val) => val.get_row_id(),
            Self::Recording(val) => val.get_row_id(),
            Self::Release(val) => val.get_row_id(),
            Self::Work(val) => val.get_row_id(),
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
            Self::Work(val) => val.get_mbid(),
        }
    }
}

impl MainEntity {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        let out = match self {
            MainEntity::Artist(val) => val.pretty_format(listenbrainz).await?,
            MainEntity::Label(val) => val.pretty_format().await?,
            MainEntity::Recording(val) => {
                val.pretty_format_with_credits(conn, client, listenbrainz)
                    .await?
            }
            MainEntity::Release(val) => {
                val.pretty_format_with_credits(conn, client, listenbrainz)
                    .await?
            }
            MainEntity::Work(val) => val.pretty_format().await?,
        };

        Ok(out)
    }
}
