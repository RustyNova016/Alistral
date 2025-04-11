use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::datastructures::entity_with_listens::artist::collection::artist_with_recordings::ArtistWithRecordingsCollection;
use alistral_core::datastructures::entity_with_listens::artist::collection::artist_with_recordings::ArtistWithRecordingsStrategy;
use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithListensCollection;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::release::collection::ReleaseWithListensStrategy;
use alistral_core::datastructures::entity_with_listens::release_group::collection::ReleaseGroupWithListensStrategy;
use alistral_core::datastructures::entity_with_listens::work::collection::work_with_recordings::WorkWithRecordingsStrategy;
use alistral_core::datastructures::entity_with_listens::work::collection::WorkWithListenStrategy;

use crate::models::client::AlistralCliClient;

// === Default strategies ===

pub fn artist_strategy(client: &AlistralCliClient) -> ArtistWithListenStrategy {
    ArtistWithListenStrategy::new(client.core.as_ref(), recording_strategy(client))
}

pub fn artist_recording_strategy(client: &AlistralCliClient) -> ArtistWithRecordingsStrategy {
    ArtistWithRecordingsStrategy::new(client.core.as_ref(), recording_strategy(client))
}

pub fn recording_strategy(client: &AlistralCliClient) -> RecordingWithListenStrategy {
    RecordingWithListenStrategy::new(client.core.as_ref())
}

pub fn release_strategy(client: &AlistralCliClient) -> ReleaseWithListensStrategy {
    ReleaseWithListensStrategy::new(client.core.as_ref(), recording_strategy(client))
}

pub fn release_group_strategy(client: &AlistralCliClient) -> ReleaseGroupWithListensStrategy {
    ReleaseGroupWithListensStrategy::new(client.core.as_ref(), release_strategy(client))
}

pub fn work_strategy(client: &AlistralCliClient) -> WorkWithListenStrategy {
    WorkWithListenStrategy::new(client.core.as_ref(), recording_strategy(client))
}

pub fn work_recording_strategy(client: &AlistralCliClient) -> WorkWithRecordingsStrategy {
    WorkWithRecordingsStrategy::new(client.core.as_ref(), recording_strategy(client))
}

// === Default stats fetching ===

pub async fn artist_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<ArtistWithListensCollection, crate::Error> {
    let strategy = artist_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

pub async fn artist_recording_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<ArtistWithRecordingsCollection, crate::Error> {
    let strategy = artist_recording_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}

pub async fn recording_stats(
    client: &AlistralCliClient,
    user: String,
) -> Result<RecordingWithListensCollection, crate::Error> {
    let strategy = recording_strategy(client);

    Ok(ListenFetchQuery::get_entity_with_listens(&client.core, user, &strategy).await?)
}
