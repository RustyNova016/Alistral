use std::collections::HashMap;

use derive_getters::Getters;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use musicbrainz_db_lite::RowId;

use crate::database::listenbrainz::prefetching::prefetch_releases;
use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;

use super::release_with_listens::ReleaseWithListens;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
pub struct ReleaseGroupWithListens<T: ListenCollectionLike> {
    release_group: ReleaseGroup,
    listens: HashMap<i64, T>,
}

impl<T: ListenCollectionLike> ReleaseGroupWithListens<T> {
    /// Insert a listen element
    fn insert_element(&mut self, id: i64, element: T) {
        self.listens.insert(id, element);
    }
}

impl ReleaseGroupWithListens<ReleaseWithListens> {
    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<HashMap<i64, Self>, crate::Error> {
        // Convert Releases
        let releases = ReleaseWithListens::from_listencollection(conn, listens).await?;

        // Prefetch the releases
        let recording_refs = releases.values().map(|r| r.release()).collect_vec();
        prefetch_releases(conn, &recording_refs).await?;

        // Load Releases
        let results = Release::get_release_groups_as_batch(conn, &recording_refs).await?;

        // Convert releases
        let mut out = HashMap::new();

        for (_, (release, release_groups)) in results {
            for release_group in release_groups {
                let release_with_listens = releases.get(&release.id).expect("The release group has been fetched from the release, so it should be there").clone();
                out.entry(release_group.get_row_id())
                    .or_insert_with(|| Self {
                        release_group,
                        listens: HashMap::new(),
                    })
                    .insert_element(
                        release_with_listens.release().get_row_id(),
                        release_with_listens,
                    );
            }
        }

        Ok(out)
    }
}

impl ListenCollectionLike for ReleaseGroupWithListens<ReleaseWithListens> {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.listens.values().flat_map(|l| l.iter_listens())
    }
}
