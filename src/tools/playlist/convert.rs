use clap::Parser;
use interzic::models::services::listenbrainz::Listenbrainz;
use interzic::models::services::youtube::Youtube;
use tuillez::fatal_error::IntoFatal;

use crate::ALISTRAL_CLIENT;
use crate::tools::playlist::PlaylistOrigin;
use crate::tools::playlist::PlaylistTarget;

#[derive(Parser, Debug, Clone)]
pub struct PlaylistConvertCommand {
    /// Get the playlist from which service?
    pub source: PlaylistOrigin,

    /// The id of the playlist on the external service
    pub id: String,

    /// Convert to this service
    pub target: PlaylistTarget,
}

impl PlaylistConvertCommand {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let playlist = match self.source {
            PlaylistOrigin::Listenbrainz => {
                Listenbrainz::import_playlist(&ALISTRAL_CLIENT.interzic, &self.id)
                    .expect_fatal("Couldn't retrieve the playlist. Check for typos.")?
            }
        };

        let playlist = playlist
            .save_recordings(&ALISTRAL_CLIENT.interzic)
            .await
            .expect_fatal("Couldn't save the playlist's recording")?;

        match self.target {
            PlaylistTarget::Youtube => {
                Youtube::create_playlist(&ALISTRAL_CLIENT.interzic, playlist)
                    .await
                    .expect_fatal("Couldn't send the playlist to youtube")?;
            }
        }

        Ok(())
    }
}
