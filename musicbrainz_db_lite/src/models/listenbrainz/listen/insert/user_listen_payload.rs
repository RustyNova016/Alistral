use listenbrainz_rs::api::user::username::listens::UserListensListen;
use sequelles::InsertOrIgnore as _;
use sequelles::Selsert as _;
use snafu::ResultExt;
use snafu::Snafu;

use crate::MBIDRedirection as _;
use crate::Recording;
use crate::error::sqlx_error::SqlxError;
use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmissionInsert;
use crate::models::listenbrainz::messybrainz_submission::MessybrainzSubmissionSqlError;
use crate::models::listenbrainz::msid_mapping::MsidMapping;
use crate::models::musicbrainz::user::UserInsert;
use crate::models::musicbrainz::user::UserSqlError;

impl Listen {
    pub async fn insert_user_listen_listen(
        conn: &mut sqlx::SqliteConnection,
        listen: UserListensListen,
    ) -> Result<Listen, UserListensListenInsertError> {
        // First, get the user
        let user = UserInsert::builder()
            .name(listen.user_name.to_owned())
            .build()
            .selsert(&mut *conn)
            .await
            .context(UserSqlSnafu)?;

        // Then upsert the MSID.
        MessybrainzSubmissionInsert::builder()
            .artist_credit(listen.track_metadata.artist_name)
            .recording(listen.track_metadata.track_name)
            .maybe_release(listen.track_metadata.release_name)
            .msid(listen.recording_msid.clone())
            .build()
            .insert_or_ignore(&mut *conn)
            .await
            .context(MessybrainzSubmissionSqlSnafu)?;

        // Set the mapping if available
        if let Some(mapping) = &listen.track_metadata.mbid_mapping {
            // First insert the mbid
            Recording::add_redirect_mbid(conn, &mapping.recording_mbid)
                .await
                .context(AddRecordingRedirectSnafu)?;

            MsidMapping::set_user_mapping(
                &mut *conn,
                user.id,
                listen.recording_msid.clone(),
                mapping.recording_mbid.clone(),
            )
            .await
            .context(SetUserMappingSnafu)?;
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

        listen_db
            .upsert_listen(conn)
            .await
            .context(ListenUpsertSnafu)
    }
}

#[derive(Debug, Snafu)]
pub enum UserListensListenInsertError {
    UserSqlError {
        source: UserSqlError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    MessybrainzSubmissionSqlError {
        source: MessybrainzSubmissionSqlError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    AddRecordingRedirectError {
        source: sqlx::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    SetUserMappingError {
        source: sqlx::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ListenUpsertError {
        source: SqlxError,

        #[snafu(implicit)]
        location: snafu::Location,
    },
}
