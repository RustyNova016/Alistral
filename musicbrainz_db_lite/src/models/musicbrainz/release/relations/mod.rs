use std::sync::Arc;

use futures::SinkExt as _;
use futures::channel::mpsc::Sender;
use sqlx::SqliteConnection;

use crate::DBClient;
use crate::FetchAsComplete;
use crate::models::musicbrainz::main_entities::MainEntity;

use super::Media;
use super::Release;

pub mod label_infos;
pub mod labels;
pub mod recording;
pub mod release_group;

impl Release {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_medias_or_fetch(
        &self,
        conn: &mut SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Media>, crate::Error> {
        // First, make sure all the data of the entity is in the database
        let id = self.get_or_fetch_as_complete(conn, client).await?.id;

        // Next, get all the children
        Ok(sqlx::query_as(
            r#"SELECT
                    medias.*
                FROM
                    releases
                    INNER JOIN medias ON releases.id = medias.release
                WHERE
                    releases.id = ?"#,
        )
        .bind(id)
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        Self::fetch_mbid_as_complete_as_task(client.clone(), &self.mbid).await?;

        let recordings = self
            .get_recordings_or_fetch(&mut *client.get_raw_connection().await?, &client)
            .await?;
        for recording in recordings {
            sender
                .send(Arc::new(MainEntity::Recording(recording)))
                .await?;
        }

        Ok(())
    }
}
