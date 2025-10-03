use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::shared_traits::db_relation::ArtistFromCreditsRelation;
use tracing::instrument;
use tuillez::pg_spinner;
use tuillez::tracing_utils::pg_future::PGFuture as _;

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
    recording_strat: RecordingWithListenStrategy,
}

impl<'l> ArtistWithRecordingsStrategy<'l> {
    pub fn new(client: &'l AlistralClient, recording_strat: RecordingWithListenStrategy) -> Self {
        Self {
            client,
            recording_strat,
        }
    }
}

impl ListenSortingStrategy<Artist, RecordingWithListensCollection>
    for ArtistWithRecordingsStrategy<'_>
{
    #[instrument(skip(self, client, data, listens), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Artist, RecordingWithListensCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling artist listen data");
        // Convert Recordings
        let recording_listens =
            RecordingWithListensCollection::from_listens(client, listens, &self.recording_strat)
                .await?;

        let recording_refs = recording_listens.iter_entities().collect_vec();

        // Fetch
        fetch_artists_of_recordings(self.client, &recording_refs).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Load artists
        let results =
            Recording::get_related_entity_bulk::<ArtistFromCreditsRelation>(conn, &recording_refs)
                .pg_spinner("Loading recordings from cache...")
                .await?;

        let results = results
            .into_many_to_many(recording_refs)
            .into_many_to_zero_right()
            .map_right(|right| {
                recording_listens
                    .get_by_id(right.rowid())
                    .cloned()
                    .expect("Error while compiling. Got a recording that isn't in the stats")
            });

        // Convert artists
        for (left, right) in results {
            // Insert the listens into the data structure
            if let Some(left) = left {
                data.insert_or_merge_entity_stats(ArtistWithRecordings {
                    entity: left,
                    listens: right.into(),
                });
            }
        }
        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        client: &AlistralClient,
        data: &mut EntityWithListensCollection<Artist, RecordingWithListensCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, client, data, vec![listen]).await
    }
}
