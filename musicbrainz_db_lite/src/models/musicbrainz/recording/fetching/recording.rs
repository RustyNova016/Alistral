use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use streamies::TryStreamies;

use crate::FetchAsComplete as _;
use crate::Recording;

impl Recording {
    pub async fn bulk_fetch_as_complete(
        client: Arc<crate::DBClient>,
        entities: Vec<Self>,
    ) -> Result<Vec<Self>, crate::Error> {
        stream::iter(entities)
            .map(async |ent| ent.fetch_as_complete_as_task(client.clone()).await)
            .buffered(8)
            .try_collect_vec()
            .await
    }
}
