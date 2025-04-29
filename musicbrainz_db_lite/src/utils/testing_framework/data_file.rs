use musicbrainz_rs_nova::entity::artist::Artist as MBArtist;
use musicbrainz_rs_nova::entity::recording::Recording as MSRecording;
use serde::Deserialize;
use serde::Serialize;

use crate::Artist;
use crate::Recording;
use crate::SaveFrom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFile {
    artists: Option<Vec<MBArtist>>,
    recordings: Option<Vec<MSRecording>>,
}

impl DataFile {
    pub async fn save_datafile(
        self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), crate::Error> {
        for data in self.artists.unwrap_or_default() {
            Artist::save_from(conn, data).await?;
        }

        for data in self.recordings.unwrap_or_default() {
            Recording::save_from(conn, data).await?;
        }

        Ok(())
    }
}
