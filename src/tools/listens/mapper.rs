use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::models::musicbrainz::user::User;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::utils::listenbrainz_api::map_msid_to_mbid;

pub async fn listen_mapper_convert_mbids(
    conn: &mut sqlx::SqliteConnection,
    original_mbid: &str,
    new_mbid: &str,
    username: &str,
    token: &str,
) {
    ListenFetchQuery::builder()
        .fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::None)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch listens");

    let user = User::find_by_name(conn, username)
        .await
        .expect("Error while getting the user")
        .expect("Couldn't find user");
    let msids = MessybrainzSubmission::get_messybrainzs_from_mbid(conn, original_mbid, user.id)
        .await
        .expect("Couldn't get the MSIDs associated to the MBID");

    for msid in &msids {
        map_msid_to_mbid(&msid.msid, new_mbid, token)
            .await
            .expect("Couldn't remap the msid");

        let listens = MessybrainzSubmission::get_listens_of_msid(conn, &msid.msid)
            .await
            .expect("Couldn't get the listens of the msid");
        if let Some(listen) = listens.first() {
            // Check if we are refreshing the listen of the current user.
            // Refreshing other is useless as they weren't modified by this function
            if listen.user.to_lowercase() == username.to_lowercase() {
                Listen::fetch_listen_by_id(
                    conn,
                    &ALISTRAL_CLIENT.listenbrainz,
                    listen.listened_at,
                    &listen.user,
                    &listen.recording_msid,
                    20,
                )
                .await
                .expect("Couldn't refresh listen")
                .expect("Couldn't refresh listen");
            }
        }
    }

    println!("Remapped {} msids", msids.len());
}
