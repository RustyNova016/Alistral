use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::work::Work;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::work::work_with_recordings::WorkWithRecordings;
use crate::datastructures::listen_sorter::ListenSortingStrategy;
use crate::AlistralClient;

pub type WorkWithRecordingsCollection =
    EntityWithListensCollection<Work, RecordingWithListensCollection>;

pub struct WorkWithRecordingsStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    recording_strat: RecordingWithListenStrategy<'l>,
}

impl<'l> WorkWithRecordingsStrategy<'l> {
    pub fn new(
        client: &'l AlistralClient,
        recording_strat: RecordingWithListenStrategy<'l>,
    ) -> Self {
        Self {
            client,
            recording_strat,
        }
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
        pg_spinner!("Compiling artist listen data");
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
                data.insert_or_merge_entity(WorkWithRecordings {
                    entity: work,
                    listens: recording.into(),
                });
            }
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
