use itertools::Itertools as _;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;

use crate::database::fetching::releases::prefetch_releases;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithListensCollection;
use crate::datastructures::entity_with_listens::release_group::ReleaseGroupWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub type ReleaseGroupWithListensCollection =
    EntityWithListensCollection<ReleaseGroup, ListenCollection>;

impl ReleaseGroupWithListensCollection {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<Self, crate::Error> {
        // Convert Releases
        let releases = ReleaseWithListensCollection::from_listencollection(conn, listens).await?;

        // Prefetch the releases
        let recording_refs = releases.iter_entities().collect_vec();
        prefetch_releases(conn, &recording_refs).await?;

        // Load Releases
        let results = Release::get_release_groups_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = Self::new();

        for (_, (release, release_groups)) in results {
            for release_group in release_groups {
                let release_with_listens = releases.get_by_id(release.id).expect("The release group has been fetched from the release, so it should be there").clone();

                out.insert_or_merge_entity(ReleaseGroupWithListens {
                    entity: release_group,
                    listens: release_with_listens.listens,
                });
            }
        }

        Ok(out)
    }
}
