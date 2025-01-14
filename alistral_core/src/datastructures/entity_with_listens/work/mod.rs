use collection::WorkWithListensCollection;
use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::listen_collection::ListenCollection;
use crate::models::relations::parenting::is_relation_parent;

use super::recording::collection::RecordingWithListensCollection;
use super::EntityWithListens;

pub mod collection;

pub type WorkWithListens = EntityWithListens<Work, ListenCollection>;

impl WorkWithListens {
    pub fn work(&self) -> &Work {
        &self.entity
    }
    pub async fn from_recording_with_listens(
        conn: &mut sqlx::SqliteConnection,
        recordings: RecordingWithListensCollection,
    ) -> Result<WorkWithListensCollection, crate::Error> {
        // Prefetch Releases
        let recording_refs = recordings.iter_entities().collect_vec();
        fetch_recordings_as_complete(conn, &recording_refs).await?;

        // Load Releases
        let results = Recording::get_works_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = WorkWithListensCollection::new();

        for (_, (recording, works)) in results {
            for work in works {
                // Get listens
                let listens = recordings
                    .0
                    .get(&recording.id)
                    .expect(
                        "The release has been fetched from the recording, so it should be there",
                    )
                    .listens()
                    .clone();

                // Create the entity
                let new = Self {
                    entity: work,
                    listens,
                };

                // Save it
                out.insert_or_merge_entity(new);
            }
        }

        Ok(out)
    }

    pub async fn get_parents(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<Self>, crate::Error> {
        self.entity.fetch_if_incomplete(conn).await?;
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
