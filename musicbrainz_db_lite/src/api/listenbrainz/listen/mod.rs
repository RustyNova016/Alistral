pub mod fetching;
use crate::Error;
use crate::models::listenbrainz::msid_mapping::MsidMapping;
use crate::models::musicbrainz::recording::Recording;
use listenbrainz::raw::response::UserListensListen;
use sqlx::SqliteConnection;

use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use crate::models::{listenbrainz::listen::Listen, musicbrainz::user::User};

impl Listen {
    pub async fn insert_api_listen(
        conn: &mut SqliteConnection,
        listen: &UserListensListen,
    ) -> Result<Listen, Error> {
        // First, get the user
        User::insert_or_ignore(&mut *conn, &listen.user_name).await?;

        // Then upsert the MSID.
        MessybrainzSubmission::from(listen)
            .insert_or_ignore(&mut *conn)
            .await?;

        // Set the mapping if available
        if let Some(mapping) = &listen.track_metadata.mbid_mapping {
            // First insert the mbid
            Recording::add_redirect_mbid(conn, &mapping.recording_mbid).await?;

            let user = User::find_by_name(&mut *conn, &listen.user_name)
                .await?
                .expect("The user shall be inserted");

            MsidMapping::set_user_mapping(
                &mut *conn,
                user.id,
                listen.recording_msid.clone(),
                mapping.recording_mbid.clone(),
            )
            .await?;
        }

        let data = serde_json::to_string(&listen.track_metadata.additional_info)
            .expect("Crashing from serializing a serde::Value isn't possible");

        let listen_db = Listen {
            id: 0,
            listened_at: listen.listened_at,
            user: listen.user_name.clone(),
            recording_msid: listen.recording_msid.clone(),
            data: Some(data),
        };

        listen_db.upsert_listen(conn).await?;

        Ok(listen_db)
    }
}

impl From<&UserListensListen> for Listen {
    fn from(value: &UserListensListen) -> Self {
        Self {
            id: Default::default(),
            listened_at: value.listened_at,
            user: value.user_name.clone(),
            recording_msid: value.recording_msid.clone(),
            data: Some(
                serde_json::to_string(&value.track_metadata.additional_info)
                    .expect("Crashing from serializing a serde::Value isn't possible"),
            ),
        }
    }
}

impl From<&UserListensListen> for MessybrainzSubmission {
    fn from(value: &UserListensListen) -> Self {
        Self {
            id: Default::default(),
            msid: value.recording_msid.clone(),
            recording: value.track_metadata.track_name.clone(),
            artist_credit: value.track_metadata.artist_name.clone(),
            release: value.track_metadata.release_name.clone(),
            track_number: None, // TODO: Find where is it stored in the json... If it even is stored...
            duration: None, //TODO: Get the duration from additiona info or ditch it from the schema?
        }
    }
}
