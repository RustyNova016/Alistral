use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use clap::Parser;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use musicbrainz_db_lite::models::musicbrainz::user::User;

use crate::ALISTRAL_CLIENT;
use crate::utils::listenbrainz_api::map_msid_to_mbid;
use crate::utils::user_inputs::UserInputParser;

/// Changes all the listens of a recording into another. Useful if LB mapped to a recording you never listened
#[derive(Parser, Debug, Clone)]
pub struct ListenRemapMsidCommand {
    /// The MBID of the recording
    original_id: String,

    /// The MBID of the recorind to replace it with
    new_id: String,

    /// Your username
    username: Option<String>,

    /// Your account token
    token: Option<String>,
}

impl ListenRemapMsidCommand {
    pub async fn run(&self) {
        // Parse data
        let username = UserInputParser::username_or_default(&self.username);
        let token = UserInputParser::user_token_or_default(&self.username, &self.token);
        let original_recording = UserInputParser::parse_recording(&self.original_id).await;
        let new_recording = UserInputParser::parse_recording(&self.original_id).await;
        let conn = &mut *ALISTRAL_CLIENT.get_conn().await;

        // Make sure the recordings are mapped to the listens
        ListenStatisticsData::new_from_user_listens(ALISTRAL_CLIENT.core.clone(), username.clone())
            .await
            .unwrap();

        let user = User::find_by_name(conn, &username)
            .await
            .expect("Error while getting the user")
            .expect("Couldn't find user");
        let msids = MessybrainzSubmission::get_messybrainzs_from_mbid(
            conn,
            &original_recording.mbid,
            user.id,
        )
        .await
        .expect("Couldn't get the MSIDs associated to the MBID");

        for msid in &msids {
            map_msid_to_mbid(&msid.msid, &new_recording.mbid, &token)
                .await
                .expect("Couldn't remap the msid");

            let listens = MessybrainzSubmission::get_listens_of_msid(conn, &msid.msid)
                .await
                .expect("Couldn't get the listens of the msid");
            if let Some(listen) = listens.first() {
                // Check if we are refreshing the listen of the current user.
                // Refreshing other is useless as they weren't modified by this function
                if listen.user.to_lowercase() == username.to_lowercase() {
                    Listen::fetch_and_insert_by_index(
                        &ALISTRAL_CLIENT.musicbrainz_db,
                        listen.listened_at,
                        &listen.user,
                        &listen.recording_msid,
                    )
                    .await
                    .expect("Couldn't refresh listen")
                    .expect("Couldn't refresh listen");
                }
            }
        }

        println!("Remapped {} msids", msids.len());
    }
}
