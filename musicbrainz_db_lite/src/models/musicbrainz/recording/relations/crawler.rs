use std::sync::Arc;

use futures::SinkExt as _;
use futures::channel::mpsc::Sender;

use crate::DBClient;
use crate::FetchAsComplete;
use crate::MainEntity;
use crate::Recording;
use crate::models::shared_traits::db_relation::ArtistFromCreditsRelation;

impl Recording {
    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        let this = self.fetch_as_complete_as_task(client.clone()).await?;

        let artists = this
            .get_related_entity_or_fetch_as_task::<ArtistFromCreditsRelation>(&client)
            .await?;
        for artist in artists {
            sender.send(Arc::new(MainEntity::Artist(artist))).await?;
        }

        let releases = this
            .get_releases_or_fetch(&mut *client.get_raw_connection().await?, &client)
            .await?;
        for release in releases {
            sender.send(Arc::new(MainEntity::Release(release))).await?;
        }

        let works = this
            .get_works_or_fetch(&mut *client.get_raw_connection().await?, &client)
            .await?;
        for work in works {
            sender.send(Arc::new(MainEntity::Work(work))).await?;
        }

        Ok(())
    }
}
