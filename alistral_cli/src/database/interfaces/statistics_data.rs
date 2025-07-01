use alistral_core::database::fetching::listens::ListenFetchQuery;
#[cfg(feature = "stats")]
use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsStrategy;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
#[cfg(feature = "stats")]
use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsCollection;
#[cfg(feature = "stats")]
use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithRecordingsStrategy;
#[cfg(feature = "stats")]
use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesCollection;
#[cfg(feature = "stats")]
use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithReleasesStrategy;
// use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithRecordingsCollection;
// use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithRecordingsStrategy;

use crate::models::client::AlistralCliClient;

// === Default strategies ===

pub fn artist_strategy(client: &AlistralCliClient) -> ArtistWithRecordingsStrategy<'_> {
    ArtistWithRecordingsStrategy::new(client.core.as_ref(), recording_strategy(client))
}

pub fn recording_strategy(client: &AlistralCliClient) -> RecordingWithListenStrategy<'_> {
    RecordingWithListenStrategy::new(client.core.as_ref())
}

#[cfg(feature = "stats")]
pub fn release_strategy(client: &AlistralCliClient) -> ReleaseWithRecordingsStrategy<'_> {
    ReleaseWithRecordingsStrategy::new(client.core.as_ref(), recording_strategy(client))
}

#[cfg(feature = "stats")]
pub fn release_group_strategy(client: &AlistralCliClient) -> ReleaseGroupWithReleasesStrategy<'_> {
    ReleaseGroupWithReleasesStrategy::new(client.core.as_ref(), release_strategy(client))
}

// pub fn work_strategy(client: &AlistralCliClient) -> WorkWithRecordingsStrategy<'_> {
//     WorkWithRecordingsStrategy::new(client.core.as_ref(), recording_strategy(client))
// }

// === Default stats fetching ===

#[cfg(feature = "stats")]
pub async fn artist_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<ArtistWithRecordingsCollection, crate::Error> {
    let strategy = artist_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

pub async fn recording_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<RecordingWithListensCollection, crate::Error> {
    let strategy = recording_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

#[cfg(feature = "stats")]
pub async fn release_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<ReleaseWithRecordingsCollection, crate::Error> {
    let strategy = release_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

#[cfg(feature = "stats")]
pub async fn release_group_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<ReleaseGroupWithReleasesCollection, crate::Error> {
    let strategy = release_group_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

// pub async fn work_stats(
//     client: &AlistralCliClient,
//     user: String,
// ) -> Result<WorkWithRecordingsCollection, crate::Error> {
//     let strategy = work_strategy(client);

//     Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
// }
