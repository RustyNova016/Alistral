use std::sync::Arc;

use futures::SinkExt as _;
use futures::channel::mpsc::Sender;

use crate::DBClient;
use crate::MainEntity;
use crate::Work;
use crate::models::shared_traits::completeness::FetchAsComplete;
use crate::models::shared_traits::db_relation::RecordingWorkDBRel;

impl Work {
    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        Self::fetch_mbid_as_complete_as_task(client.clone(), &self.mbid).await?;

        let recordings = self
            .get_related_entity_or_fetch_as_task::<RecordingWorkDBRel>(&client)
            .await?;
        for recording in recordings {
            sender
                .send(Arc::new(MainEntity::Recording(recording)))
                .await?;
        }

        Ok(())
    }
}
