use musicbrainz_db_lite::FetchAsComplete;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;
use crate::models::relations::parenting::is_relation_parent;

use super::EntityWithListens;

pub mod collection;

pub type WorkWithListens = EntityWithListens<Work, ListenCollection>;

pub type WorkWithRecordings = EntityWithListens<Work, RecordingWithListensCollection>;

impl WorkWithRecordings {
    pub fn work(&self) -> &Work {
        &self.entity
    }

    pub async fn get_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
    ) -> Result<Vec<Self>, crate::Error> {
        self.entity
            .fetch_as_complete_with_conn(conn, &client.musicbrainz_db)
            .await?;
        let relations = self.entity.get_work_relations(conn).await?;
        let mut out = Vec::new();

        for relation in relations {
            if is_relation_parent(&relation, self.entity.id) {
                out.push(WorkWithRecordings {
                    entity: relation.get_other_entity(conn, self.entity.id).await?,
                    listens: self.listens.clone(),
                });
            }
        }

        Ok(out)
    }
}
