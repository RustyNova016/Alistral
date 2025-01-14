use crate::database::get_conn;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::recording_with_listens::collection::RecordingWithListensCollection;
use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;

pub async fn get_test_user_listens() -> ListenCollection {
    ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user("RustyNova")
        .build()
        .fetch(&mut *get_conn().await)
        .await
        .expect("Couldn't fetch test listens")
}

pub async fn get_test_user_recording_with_listens() -> RecordingWithListensCollection {
    RecordingWithListens::from_listencollection(
        &mut *get_conn().await,
        get_test_user_listens().await,
    )
    .await
    .expect("Couldn't get test Recording with listens")
}
