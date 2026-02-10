use std::sync::Arc;

use async_once_cell::OnceCell;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::listen::selects::error::ListenFetchGetError;

use crate::AlistralClient;
use crate::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use crate::datastructures::entity_with_listens::label::collection::LabelWithReleasesCollection;
use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
use crate::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
use crate::datastructures::entity_with_listens::user::collection::UserWithListensCollection;
use crate::datastructures::listen_collection::ListenCollection;

pub mod artists;
pub mod labels;
pub mod listens;
pub mod recordings;
pub mod release;
pub mod release_groups;
pub mod user;

/// This struct hold listens data. This is a convenience over having a to create listen statistics yourself
///
/// The design is intentionally not mutable. Having a mutation somewhere would need invalidating all of the inner statistics,
/// thus having to recreate the whole struct. This means that:
/// - The inner listens are fixed
/// - The inner client is fixed
///
/// To change any of those two, you need to create a new [`ListenStatisticsData`] struct with the new parameters.
pub struct ListenStatisticsData {
    client: Arc<AlistralClient>,
    listens: ListenCollection,

    artists: OnceCell<ArtistWithRecordingsCollection>,
    labels: OnceCell<LabelWithReleasesCollection>,
    recordings: OnceCell<RecordingWithListensCollection>,
    releases: OnceCell<ReleaseWithRecordingsCollection>,
    release_groups: OnceCell<ReleaseGroupWithReleasesCollection>,
    users: OnceCell<UserWithListensCollection>,
}

impl ListenStatisticsData {
    pub fn new(client: Arc<AlistralClient>, listens: ListenCollection) -> Self {
        Self {
            client,
            listens,
            artists: OnceCell::new(),
            labels: OnceCell::new(),
            users: OnceCell::new(),
            recordings: OnceCell::new(),
            releases: OnceCell::new(),
            release_groups: OnceCell::new(),
        }
    }

    pub async fn new_from_user_listens(
        client: Arc<AlistralClient>,
        name: String,
    ) -> Result<Self, ListenFetchGetError> {
        let listens = Listen::get_or_fetch_listens()
            .client(&client.musicbrainz_db)
            .incremental(true)
            .users(&[&name])
            .mapped(true)
            .unmapped(true)
            .call()
            .await?;

        Ok(Self::new(client, listens.into()))
    }

    pub fn client(&self) -> &Arc<AlistralClient> {
        &self.client
    }

    pub fn listens(&self) -> &ListenCollection {
        &self.listens
    }

    /// Clone self while only keeping the listens.
    pub fn clone_no_stats(&self) -> Self {
        Self::new(self.client.clone(), self.listens.clone())
    }
}
