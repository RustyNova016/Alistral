// use alistral_core::database::fetching::listens::ListenFetchQuery;
// use alistral_core::database::fetching::listens::ListenFetchQueryReturn;
// use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
// use alistral_core::datastructures::listen_collection::ListenCollection;

// use crate::ALISTRAL_CLIENT;
// use crate::database::interfaces::statistics_data::recording_strategy;

// pub async fn get_test_user_listens() -> ListenCollection {
//     ListenFetchQuery::builder()
//         .fetch_recordings_redirects(true)
//         .returns(ListenFetchQueryReturn::Mapped)
//         .user("RustyNova")
//         .build()
//         .fetch(
//             &mut ALISTRAL_CLIENT
//                 .musicbrainz_db
//                 .get_raw_connection()
//                 .await
//                 .expect("Couldn't connect to the database"),
//             &ALISTRAL_CLIENT.core,
//         )
//         .await
//         .expect("Couldn't fetch test listens")
// }

// pub async fn get_test_user_recording_with_listens() -> RecordingWithListensCollection {
//     RecordingWithListensCollection::from_listencollection(
//         get_test_user_listens().await,
//         &recording_strategy(&ALISTRAL_CLIENT),
//     )
//     .await
//     .expect("Couldn't get test Recording with listens")
// }
