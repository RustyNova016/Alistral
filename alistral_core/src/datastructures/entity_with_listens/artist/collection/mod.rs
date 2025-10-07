use itertools::Itertools as _;
use musicbrainz_db_lite::HasRowID as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use sequelles::JoinCollection;
use sequelles::JoinRelation;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::artist::artist_with_recordings::ArtistWithRecordings;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy;

pub mod sort;

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

        let recording_refs = recording_listens.iter_entities().cloned().collect_vec();

        // Fetch
        let joins = Recording::fetch_all_artists_from_credits_bulk(
            self.client.musicbrainz_db.clone(),
            recording_refs.clone(),
        )
        .await?;

        let mut join_col = JoinCollection::default();
        for (recording, artists) in joins {
            for artist in artists {
                join_col.push(JoinRelation::new(artist, recording.rowid()));
            }
        }

        let results = join_col
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

