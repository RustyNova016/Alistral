use std::collections::HashMap;

use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use musicbrainz_db_lite::RowId;

use crate::database::listenbrainz::prefetching::fetch_recordings_as_complete;
use crate::datastructures::listen_collection::ListenCollection;

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
