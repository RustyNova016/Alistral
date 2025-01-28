use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::database::fetching::recordings::fetch_recordings_as_complete;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::release::ReleaseWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub type ReleaseWithListensCollection = EntityWithListensCollection<Release, ListenCollection>;

impl ReleaseWithListensCollection {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::AlistralClient,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        // Convert Recordings
        let recordings =
            RecordingWithListensCollection::from_listencollection(conn, client, listens).await?;

        let recording_refs = recordings.iter_entities().collect_vec();

        fetch_recordings_as_complete(conn, client, &recording_refs).await?;

        // Load Releases
        let results = Recording::get_releases_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = Self::new();

        for (_, (recording, releases)) in results {
            for release in releases {
                let listens = recordings
                    .0
                    .get(&recording.id)
                    .expect(
                        "The release has been fetched from the recording, so it should be there",
                    )
                    .listens
                    .clone();

                out.insert_or_merge_entity(ReleaseWithListens {
                    entity: release,
                    listens,
                });
            }
        }

        Ok(out)
    }
}
