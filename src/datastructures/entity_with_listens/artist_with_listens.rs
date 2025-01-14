use std::collections::HashMap;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::datastructures::listen_collection::ListenCollection;
use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::RowId;
use rust_decimal::Decimal;

use crate::database::listenbrainz::prefetching::fetch_recordings_as_complete;

use super::recording_with_listens::RecordingWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ArtistWithListens {
    artist: Artist,
    listens: Vec<RecordingWithListens>,
}

impl ArtistWithListens {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<HashMap<i64, Self>, crate::Error> {
        // Convert Recordings
        let recordings = RecordingWithListens::from_listencollection(conn, listens).await?;

        let recording_refs = recordings.iter_recordings().collect_vec();
        fetch_recordings_as_complete(conn, &recording_refs).await?;

        // Load artists
        let results = Recording::get_artist_from_credits_as_batch(conn, &recording_refs).await?;

        // Convert artists
        let mut out = HashMap::new();

        for (_, (recording, artists)) in results {
            for artist in artists {
                out.entry(artist.get_row_id())
                    .or_insert_with(|| Self {
                        artist,
                        listens: Vec::new(),
                    })
                    .push(recordings.0.get(&recording.id).expect("The artist has been fetched from the recording, so it should be there").clone());
            }
        }

        Ok(out)
    }

    pub fn push(&mut self, value: RecordingWithListens) {
        self.listens.push(value);
    }

    /// Return the number of recordings that have been listened
    pub fn listened_recording_count(&self) -> usize {
        self.listens
            .iter()
            .filter(|r| r.listen_count() != 0)
            .collect_vec()
            .len()
    }

    /// Return the average listens count per each recording listened by the user
    pub fn average_listen_per_recordings_listened(&self) -> Decimal {
        Decimal::from(self.listen_count()) / Decimal::from(self.listened_recording_count())
    }
}

//impl_entity_with_listens!(ReleaseWithListens);

impl ListenCollectionReadable for ArtistWithListens {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.listens.iter().flat_map(|l| l.iter_listens())
    }
}
