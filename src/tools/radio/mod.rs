use interzic::models::messy_recording::MessyRecording;
use interzic::models::playlist_stub::PlaylistStub;
use interzic::models::services::listenbrainz::Listenbrainz;
use interzic::models::services::youtube::Youtube;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::models::cli::radio::RadioExportTarget;

pub mod circles;
pub mod listen_rate;
pub mod overdue;
pub mod shared;
pub mod underrated;

impl RadioExportTarget {
    pub async fn export(
        &self,
        playlist: PlaylistStub,
        username: Option<String>,
        token: Option<&str>,
    ) -> Result<(), crate::Error> {
        match self {
            RadioExportTarget::Listenbrainz => {
                Listenbrainz::create_playlist(
                    &ALISTRAL_CLIENT.interzic,
                    playlist,
                    username.ok_or(crate::Error::MissingPlaylistUserDataError(
                        "username".to_string(),
                    ))?,
                    token.ok_or(crate::Error::MissingPlaylistUserDataError(
                        "token".to_string(),
                    ))?,
                )
                .await?;
            }
            Self::Youtube => {
                let _playlist_id =
                    Youtube::create_playlist(&ALISTRAL_CLIENT.interzic, playlist).await?;
                //TODO: display url after export
            }
        }

        Ok(())
    }
}

pub(super) async fn convert_recordings(
    db_lite_conn: &mut sqlx::SqliteConnection,
    recordings: Vec<Recording>,
) -> Result<Vec<MessyRecording>, crate::Error> {
    let mut messy = Vec::new();

    for recording in recordings {
        let rec =
            MessyRecording::from_db_recording(db_lite_conn, &ALISTRAL_CLIENT.interzic, recording)
                .await?;
        let rec = rec
            .upsert(&ALISTRAL_CLIENT.interzic.database_client)
            .await?;
        messy.push(rec);
    }

    Ok(messy)
}
