use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::cli::progress_bar::global_progress_bar::PG_FETCHING;
use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::work::WorkWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub mod work_with_recordings;

pub type WorkWithListensCollection = EntityWithListensCollection<Work, ListenCollection>;

impl WorkWithListensCollection {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<WorkWithListensCollection, crate::Error> {
        let recordings =
            RecordingWithListensCollection::from_listencollection(conn, listens).await?;
        Self::from_recording_with_listens(conn, recordings).await
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
                let new = WorkWithListens {
                    entity: work,
                    listens,
                };

                // Save it
                out.insert_or_merge_entity(new);
            }
        }

        Ok(out)
    }

    pub async fn add_parents_recursive(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), crate::Error> {
        let mut queue = self.0.values().cloned().collect_vec();
        let mut seen = Vec::new();
        let mut count = queue.len() as u64;

        //TODO: Re-add progress bars
        let progress_bar = PG_FETCHING.get_submitter(queue.len() as u64);
        while let Some(work) = queue.pop() {
            if seen.contains(&work.work().mbid.clone()) {
                continue;
            }
            let new_works = work.get_parents(conn).await?;

            for new_work in new_works {
                queue.push(new_work.clone());
                self.insert_or_merge_entity(new_work);
                count += 1;
            }
            progress_bar.inc(1);
            progress_bar.set_count(count);
            seen.push(work.work().mbid.clone());
        }

        Ok(())
    }
}
