use std::sync::Arc;

use futures::SinkExt as _;
use futures::channel::mpsc::Sender;

use crate::DBClient;
use crate::MainEntity;
use crate::Track;
use crate::models::shared_traits::db_relation::ArtistFromCreditsRelation;

impl Track {
    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        let artists = self
            .get_related_entity::<ArtistFromCreditsRelation>(
                &mut *client.get_raw_connection().await?,
            )
            .await?;
        for artist in artists {
            sender.send(Arc::new(MainEntity::Artist(artist))).await?;
        }

        let release = self
            .get_release(&mut *client.get_raw_connection().await?)
            .await?;
        if let Some(release) = release {
            sender.send(Arc::new(MainEntity::Release(release))).await?;
        }

        let recording = self
            .get_recording(&mut *client.get_raw_connection().await?)
            .await?;
        if let Some(recording) = recording {
            sender
                .send(Arc::new(MainEntity::Recording(recording)))
                .await?;
        }

        Ok(())
    }
}
