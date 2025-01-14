pub mod collection;
use std::collections::HashMap;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::datastructures::listen_collection::ListenCollection;
use collection::WorkWithListensCollection;
use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use musicbrainz_db_lite::RowId;

use crate::database::listenbrainz::prefetching::fetch_recordings_as_complete;
use crate::utils::entities::relations::is_relation_parent;

use super::recording_with_listens::collection::RecordingWithListensCollection;
use super::recording_with_listens::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct WorkWithRecordingListens {
    work: Work,
    listens: Vec<RecordingWithListens>,
}

impl WorkWithRecordingListens {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<HashMap<i64, Self>, crate::Error> {
        // Convert Recordings
        let recordings = RecordingWithListens::from_listencollection(conn, listens).await?;

        // Prefetch Releases
        let recording_refs = recordings.iter_recordings().collect_vec();
        fetch_recordings_as_complete(conn, &recording_refs).await?;

        // Load Releases
        let results = Recording::get_works_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = HashMap::new();

        for (_, (recording, works)) in results {
            for work in works {
                out.entry(work.get_row_id())
                    .or_insert_with(|| Self {
                        work,
                        listens: Vec::new(),
                    })
                    .push(recordings.0.get(&recording.id).expect("The release has been fetched from the recording, so it should be there").clone());
            }
        }

        Ok(out)
    }

    pub fn push(&mut self, value: RecordingWithListens) {
        self.listens.push(value);
    }

    /// Return the listen count
    pub fn len(&self) -> usize {
        self.listens.iter().map(|r| r.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

//impl_entity_with_listens!(ReleaseWithListens);

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
        let recording_refs = recordings.iter_recordings().collect_vec();
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
                out.insert_or_merge(new);
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

impl ListenCollectionReadable for WorkWithListens {
    fn listen_count(&self) -> usize {
        self.listens.listen_count()
    }

    fn has_no_listens(&self) -> bool {
        self.listens.has_no_listens()
    }

    fn iter_listens(
        &self,
    ) -> impl Iterator<Item = &musicbrainz_db_lite::models::listenbrainz::listen::Listen> {
        self.listens.iter_listens()
    }
}
