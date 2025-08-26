use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::release::ReleaseWithRecordings;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type ReleaseWithRecordingsCollection =
    EntityWithListensCollection<Release, RecordingWithListensCollection>;

pub struct ReleaseWithRecordingsStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    recording_strat: RecordingWithListenStrategy,
}

impl<'l> ReleaseWithRecordingsStrategy<'l> {
    pub fn new(
        client: &'l AlistralClient,
        recording_strat: RecordingWithListenStrategy,
    ) -> Self {
        Self {
            client,
            recording_strat,
        }
    }
}

impl ListenSortingStrategy<Release, RecordingWithListensCollection>
    for ReleaseWithRecordingsStrategy<'_>
{
    #[instrument(skip(self, client, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Release, RecordingWithListensCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling releases listen data");
        // Convert Recordings
        let recordings =
            RecordingWithListensCollection::from_listens(client, listens, &self.recording_strat).await?;

        let recording_refs = recordings.iter_entities().collect_vec();
        fetch_recordings_as_complete(self.client, &recording_refs).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Load Releases
        let results = Recording::get_releases_as_batch(conn, &recording_refs).await?;

        // Convert releases
        for (_, (recording, releases)) in results {
            for release in releases {
                let listens = recordings
                    .0
                    .get(&recording.id)
                    .expect(
                        "The release has been fetched from the recording, so it should be there",
                    )
                    .clone();

                data.insert_or_merge_entity_stats(ReleaseWithRecordings {
                    entity: release,
                    listens: listens.into(),
                });
            }
        }

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Release, RecordingWithListensCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, client, data, vec![listen]).await
    }
}
