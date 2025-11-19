use listenbrainz_rs::api::user::username::listens::UserListensListen;
use snafu::ResultExt;

use crate::MBIDRedirection as _;
use crate::Recording;
use crate::User;
use crate::error::sqlx_error::SqlxError;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmission;
use crate::models::listenbrainz::msid_mapping::MsidMapping;

impl Listen {
    pub async fn insert_user_listen_listen(
        conn: &mut sqlx::SqliteConnection,
        listen: UserListensListen,
    ) -> Result<Listen, SqlxError> {
        // First, get the user
        User::insert_or_ignore(&mut *conn, &listen.user_name)
            .await
            .context(SqlxSnafu)?;

        // Then upsert the MSID.
        MessybrainzSubmission::builder()
            .id(0)
            .artist_credit(listen.track_metadata.artist_name)
            .recording(listen.track_metadata.track_name)
            .maybe_release(listen.track_metadata.release_name)
            .msid(listen.recording_msid.clone())
            .build()
            .insert_or_ignore(&mut *conn)
            .await
            .context(SqlxSnafu)?;

        // Set the mapping if available
        if let Some(mapping) = &listen.track_metadata.mbid_mapping {
            // First insert the mbid
            Recording::add_redirect_mbid(conn, &mapping.recording_mbid)
                .await
                .context(SqlxSnafu)?;

            let user = User::find_by_name(&mut *conn, &listen.user_name)
                .await
                .context(SqlxSnafu)?
                .expect("The user shall be inserted");

            MsidMapping::set_user_mapping(
                &mut *conn,
                user.id,
                listen.recording_msid.clone(),
                mapping.recording_mbid.clone(),
            )
            .await
            .context(SqlxSnafu)?;
        }

        let data = serde_json::to_string(&listen.track_metadata.additional_info)
            .expect("Crashing from serializing a serde::Value isn't possible");

        let listen_db = Listen {
            id: 0,
            listened_at: listen.listened_at,
            user: listen.user_name.clone(),
            recording_msid: listen.recording_msid,
            data: Some(data),
        };

        listen_db.upsert_listen(conn).await.context(SqlxSnafu)?;

        Ok(listen_db)
    }
}
