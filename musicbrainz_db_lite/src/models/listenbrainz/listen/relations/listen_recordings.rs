use std::sync::Arc;

use crate::DBClient;
use crate::DBRelation;
use crate::GetOrFetch;
use crate::Recording;
use crate::User;
use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::msid_mapping::MsidMapping;

pub struct ListenRecordingDBRel;

impl Listen {
    pub async fn get_recording_or_fetch_with_task(
        &self,
        client: Arc<DBClient>,
    ) -> Result<Option<Recording>, crate::Error> {
        let conn = &mut *client.get_conn().await?;

        let user = User::find_by_name(conn, &self.user)
            .await?
            .expect("User should be in due to foreign keys");

        let Some(msid_mapping) =
            MsidMapping::find_by_user_msid(conn, user.id, &self.recording_msid).await?
        else {
            return Ok(None);
        };

        Recording::get_or_fetch_as_task(client, &msid_mapping.recording_mbid).await
    }
}

impl DBRelation<ListenRecordingDBRel> for Listen {
    type ReturnedType = Recording;

    fn get_join_statement() -> &'static str {
        "
        INNER JOIN `users` ON `listens`.`user` = `users`.`name`
        INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
        INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid AND `users`.`id` = `msid_mapping`.`user`
        INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
        INNER JOIN recordings ON recordings_gid_redirect.new_id = recordings.id"
    }
}
