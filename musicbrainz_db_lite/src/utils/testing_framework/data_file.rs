use musicbrainz_rs::entity::artist::Artist as MBArtist;
use musicbrainz_rs::entity::recording::Recording as MSRecording;
use serde::Deserialize;
use serde::Serialize;

use crate::Artist;
use crate::CompletenessFlag as _;
use crate::MBRelease;
use crate::Recording;
use crate::Release;
use crate::SaveFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    artists: Option<Vec<MBArtist>>,
    recordings: Option<Vec<MSRecording>>,
    releases: Option<Vec<MBRelease>>,
}

impl DataFile {
    pub async fn save_datafile(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), crate::Error> {
        for data in self.artists.unwrap_or_default() {
            let mut val = Artist::save_from(conn, data).await?;
            val.set_full_update(conn).await?;
        }

        for data in self.recordings.unwrap_or_default() {
            let mut val = Recording::save_from(conn, data).await?;
            val.set_full_update(conn).await?;
        }

        for data in self.releases.unwrap_or_default() {
            let mut val = Release::save_from(conn, data).await?;
            val.set_full_update(conn).await?;
        }

        Ok(())
    }
}
