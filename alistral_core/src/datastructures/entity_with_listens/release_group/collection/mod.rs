use itertools::Itertools as _;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use tracing::instrument;
use tuillez::pg_spinner;

use crate::database::fetching::releases::prefetch_releases;
use crate::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithListensCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithListensStrategy;
use crate::datastructures::entity_with_listens::release_group::ReleaseGroupWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::datastructures::listen_sorter::ListenSortingStrategy;
use crate::AlistralClient;

pub type ReleaseGroupWithListensCollection =
    EntityWithListensCollection<ReleaseGroup, ListenCollection>;

pub struct ReleaseGroupWithListensStrategy<'l> {
    pub(super) client: &'l AlistralClient,
    release_strat: ReleaseWithListensStrategy<'l>,
}

impl<'l> ReleaseGroupWithListensStrategy<'l> {
    pub fn new(client: &'l AlistralClient, release_strat: ReleaseWithListensStrategy<'l>) -> Self {
        Self {
            client,
            release_strat,
        }
    }
}

impl ListenSortingStrategy<ReleaseGroup, ListenCollection> for ReleaseGroupWithListensStrategy<'_> {
    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn sort_insert_listens(
        &self,
        data: &mut EntityWithListensCollection<ReleaseGroup, ListenCollection>,
        listens: Vec<Listen>,
    ) -> Result<(), crate::Error> {
        pg_spinner!("Compiling release group listens data");
        // Convert Releases
        let releases =
            ReleaseWithListensCollection::from_listens(listens, &self.release_strat).await?;

        let conn = &mut *self.client.musicbrainz_db.get_raw_connection().await?;

        // Prefetch the releases
        let recording_refs = releases.iter_entities().collect_vec();
        prefetch_releases(conn, self.client, &recording_refs).await?;

        // Load Releases
        let results = Release::get_release_groups_as_batch(conn, &recording_refs).await?;

        // Convert releases groups

        for (_, (release, release_groups)) in results {
            for release_group in release_groups {
                let release_with_listens = releases.get_by_id(release.id).expect("The release group has been fetched from the release, so it should be there").clone();

                data.insert_or_merge_entity(ReleaseGroupWithListens {
                    entity: release_group,
                    listens: release_with_listens.listens,
                });
            }
        }

        Ok(())
    }

    async fn sort_insert_listen(
        &self,
        data: &mut EntityWithListensCollection<ReleaseGroup, ListenCollection>,
        listen: Listen,
    ) -> Result<(), crate::Error> {
        Self::sort_insert_listens(self, data, vec![listen]).await
    }
}
