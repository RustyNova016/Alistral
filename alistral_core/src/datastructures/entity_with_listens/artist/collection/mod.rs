use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::database::fetching::recordings::fetch_artists_of_recordings;
use crate::datastructures::entity_with_listens::artist::artist_with_recordings::ArtistWithRecordings;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub type ArtistWithRecordingsCollection =
    EntityWithListensCollection<Artist, RecordingWithListensCollection>;

pub struct ArtistWithRecordingsStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    recording_strat: RecordingWithListenStrategy<'l>,
}

impl<'l> ArtistWithRecordingsStrategy<'l> {
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

impl ListenSortingStrategy<Artist, RecordingWithListensCollection>
    for ArtistWithRecordingsStrategy<'_>
{
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        data: &mut EntityWithListensCollection<Artist, RecordingWithListensCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling artist listen data");
        // Convert Recordings
        let recordings =
            RecordingWithListensCollection::from_listens(listens, &self.recording_strat).await?;

        let recording_refs = recordings.iter_entities().collect_vec();
        fetch_artists_of_recordings(self.client, &recording_refs).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Load artists
        let results = Recording::get_artist_from_credits_as_batch(conn, &recording_refs).await?;

        // Convert artists
        for (_, (recording, artists)) in results {
            for artist in artists {
                let recording = recordings
                    .0
                    .get(&recording.id)
                    .expect("The artist has been fetched from the recording, so it should be there")
                    .clone();

                let artist_with_recordings = ArtistWithRecordings {
                    entity: artist,
                    listens: recording.into(),
                };

                data.insert_or_merge_entity_stats(artist_with_recordings);
            }
        }

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        data: &mut EntityWithListensCollection<Artist, RecordingWithListensCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, data, vec![listen]).await
    }
}
