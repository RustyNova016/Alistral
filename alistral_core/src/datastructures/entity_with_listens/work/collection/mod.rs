use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use tracing::Span;
use tracing::instrument;
use tracing_indicatif::span_ext::IndicatifSpanExt as _;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::work::WorkWithRecordings;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type WorkWithRecordingsCollection =
    EntityWithListensCollection<Work, RecordingWithListensCollection>;

pub struct WorkWithRecordingsStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    recording_strat: RecordingWithListenStrategy<'l>,
    recursive_parents: bool,
}

impl<'l> WorkWithRecordingsStrategy<'l> {
    pub fn new(
        client: &'l AlistralClient,
        recording_strat: RecordingWithListenStrategy<'l>,
    ) -> Self {
        Self {
            client,
            recording_strat,
            recursive_parents: false,
        }
    }

    pub fn with_recursive_parents(mut self) -> Self {
        self.recursive_parents = true;
        self
    }
}

impl ListenSortingStrategy<Work, RecordingWithListensCollection>
    for WorkWithRecordingsStrategy<'_>
{
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        data: &mut EntityWithListensCollection<Work, RecordingWithListensCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling work listen data");
        // Convert Recordings
        let recordings =
            RecordingWithListensCollection::from_listens(listens, &self.recording_strat).await?;

        let recording_refs = recordings.iter_entities().collect_vec();
        fetch_recordings_as_complete(self.client, &recording_refs).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Load artists
        let results = Recording::get_works_as_batch(conn, &recording_refs).await?;

        // Convert artists
        for (_, (recording, works)) in results {
            for work in works {
                let recording = recordings
                    .0
                    .get(&recording.id)
                    .expect(
                        "The release has been fetched from the recording, so it should be there",
                    )
                    .clone();

                data.insert_or_merge_entity_stats(WorkWithRecordings {
                    entity: work,
                    listens: recording.into(),
                });
            }
        }

        if self.recursive_parents {
            data.add_parents_recursive(conn, self.client).await?;
        }

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        data: &mut EntityWithListensCollection<Work, RecordingWithListensCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, data, vec![listen]).await
    }
}

impl WorkWithRecordingsCollection {
    #[instrument(skip(client), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn add_parents_recursive(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
    ) -> Result<(), crate::Error> {
        let mut queue = self.0.values().cloned().collect_vec();
        let mut seen = Vec::new();
        let mut count = queue.len() as u64;
        pg_counted!(queue.len(), "Fetching parent works");

        while let Some(work) = queue.pop() {
            if seen.contains(&work.work().mbid.clone()) {
                continue;
            }
            let new_works = work.get_parents(conn, client).await?;

            for new_work in new_works {
                queue.push(new_work.clone());
                self.insert_or_merge_entity_stats(new_work);
                count += 1;
            }
            pg_inc!();
            Span::current().pb_set_length(count);
            seen.push(work.work().mbid.clone());
        }

        Ok(())
    }
}
