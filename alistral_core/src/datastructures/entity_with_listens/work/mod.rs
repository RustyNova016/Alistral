use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::datastructures::listen_collection::ListenCollection;
use crate::models::relations::parenting::is_relation_parent;

use super::EntityWithListens;

pub mod collection;
pub mod work_with_recordings;

pub type WorkWithListens = EntityWithListens<Work, ListenCollection>;

impl WorkWithListens {
    pub fn work(&self) -> &Work {
        &self.entity
    }

    pub async fn get_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
    ) -> Result<Vec<Self>, crate::Error> {
        self.entity
            .fetch_if_incomplete(conn, &client.musicbrainz_db)
            .await?;
        let relations = self.entity.get_work_relations(conn).await?;
        let mut out = Vec::new();

        for relation in relations {
            if is_relation_parent(&relation, self.entity.id) {
                //TODO: Proper relation checking
                out.push(WorkWithListens {
                    entity: relation.get_other_entity(conn, self.entity.id).await?,
                    listens: self.listens.clone(),
                });
            }
        }

        Ok(out)
    }
}
