use clap::Parser;
use interzic::models::services::listenbrainz::Listenbrainz;
use tuillez::fatal_error::IntoFatal as _;

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

    #[clap(short, long, default_value_t)]
    pub instance: String,

    /// Use the mapping of this user
    pub user: Option<String>,
}

impl PlaylistConvertCommand {
    #[cfg(not(any(feature = "subsonic")))]
    pub async fn run(&self) -> Result<(), crate::Error> {
        Ok(())
    }

    #[cfg(feature = "subsonic")]
    pub async fn run(&self) -> Result<(), crate::Error> {
        #[cfg(feature = "subsonic")]
        let client_name = self.instance.to_lowercase();

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
            #[cfg(feature = "subsonic")]
            PlaylistTarget::Subsonic => {
                let Some(client) = ALISTRAL_CLIENT.interzic.get_subsonic_client(&client_name)
                else {
                    use tracing::error;

                    error!(
                        "Couldn't find the subsonic server with name `{client_name}`. You may want to add one with `alistral interzic add-subsonic`"
                    );
                    return Ok(());
                };

                client
                    .create_playlist(&ALISTRAL_CLIENT.interzic, playlist, self.user.clone())
                    .await
                    .unwrap();
            }
        }

        Ok(())
    }
}
