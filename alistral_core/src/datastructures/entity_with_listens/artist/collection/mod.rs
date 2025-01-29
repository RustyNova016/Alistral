pub mod artist_with_recordings;
use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::artist::Artist;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use tracing::instrument;

use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::artist::ArtistWithListens;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;
use crate::pg_spinner;

pub type ArtistWithListensCollection = EntityWithListensCollection<Artist, ListenCollection>;

impl ArtistWithListensCollection {
    #[instrument(skip_all, fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        pg_spinner!("Compiling artist data");
        // Convert Recordings
        let recordings =
            RecordingWithListensCollection::from_listencollection(conn, client, listens).await?;

        let recording_refs = recordings.iter_entities().collect_vec();
        fetch_recordings_as_complete(conn, client, &recording_refs).await?;

        // Load artists
        let results = Recording::get_artist_from_credits_as_batch(conn, &recording_refs).await?;

        // Convert artists
        let mut out = Self::new();

        for (_, (recording, artists)) in results {
            for artist in artists {
                let listens = recordings
                    .0
                    .get(&recording.id)
                    .expect("The artist has been fetched from the recording, so it should be there")
                    .listens
                    .clone();

                out.insert_or_merge_entity(ArtistWithListens {
                    entity: artist,
                    listens,
                });
            }
        }

        Ok(out)
    }
}
