use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;

use crate::api::clients::ALISTRAL_CLIENT;

pub async fn get_test_user_listens() -> ListenCollection {
    ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user("RustyNova")
        .build()
        .fetch(
            &mut *ALISTRAL_CLIENT
                .musicbrainz_db
                .connection
                .acquire_guarded()
                .await,
            &ALISTRAL_CLIENT,
        )
        .await
        .expect("Couldn't fetch test listens")
}

pub async fn get_test_user_recording_with_listens() -> RecordingWithListensCollection {
    RecordingWithListensCollection::from_listencollection(
        &mut *ALISTRAL_CLIENT
            .musicbrainz_db
            .connection
            .acquire_guarded()
            .await,
        &ALISTRAL_CLIENT,
        get_test_user_listens().await,
    )
    .await
    .expect("Couldn't get test Recording with listens")
}
