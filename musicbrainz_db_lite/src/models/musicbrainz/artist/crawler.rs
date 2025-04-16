use std::sync::Arc;

use futures::channel::mpsc::Sender;
use futures::pin_mut;
use futures::SinkExt as _;
use futures::TryStreamExt;

use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::main_entities::MainEntity;
use crate::DBClient;

impl Artist {
    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        let conn = &mut *client.get_raw_connection().await?;
        let rec_stream = self.browse_or_fetch_artist_recordings(conn, client);

        pin_mut!(rec_stream);

        while let Some(recording) = rec_stream.try_next().await? {
            sender
                .send(Arc::new(MainEntity::Recording(recording)))
                .await?
        }

        Ok(())
    }
}
