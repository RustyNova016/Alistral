use std::sync::Arc;

use crate::CompletenessFlag;
use crate::DBClient;
use crate::FetchAndSave as _;
use crate::HasMBID;
use crate::models::musicbrainz::track::Track;

impl Track {
    pub async fn refetch_as_task(&self, client: Arc<DBClient>) -> Result<Self, crate::Error> {
        let release = self
            .get_release(&mut *client.get_raw_connection().await?)
            .await?
            .ok_or(crate::Error::NotFoundInUpstream(
                self.get_mbid().to_string(),
            ))?;

        release.refetch_as_task(client.clone()).await?;

        Self::find_by_mbid(&mut *client.get_raw_connection().await?, self.gid.clone())
            .await?
            .ok_or(crate::Error::NotFoundInUpstream(
                self.get_mbid().to_string(),
            ))
    }
}

impl CompletenessFlag for Track {
    async fn set_full_update(
        &mut self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }

    fn is_complete(&self) -> bool {
        // You can't have a partial track
        true
    }
}
