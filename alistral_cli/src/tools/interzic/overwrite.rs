use clap::Parser;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::youtube::Youtube;
use tuillez::fatal_error::FatalError;
use tuillez::fatal_error::IntoFatal as _;
use tuillez::fatal_error::OptIntoFatal as _;

use crate::ALISTRAL_CLIENT;
use crate::tools::interzic::get_mapping::InterzicMappingTarget;
use crate::utils::cli::read_mbid_from_input;

#[derive(Parser, Debug, Clone)]
pub struct OverwriteCommand {
    /// Set the mapping of this service
    pub target: InterzicMappingTarget,

    /// Set the mapping of this user
    pub user: String,

    /// The id on the external service
    pub id: String,

    /// The title of the recording
    #[arg(short, long)]
    pub recording: Option<String>,

    /// The artist credit
    #[arg(short, long)]
    pub artist: Option<String>,

    /// The title of the release
    #[arg(short = 'g', long)]
    pub release: Option<String>,

    /// The mbid to search from (Overwrite `recording`, `artist`, and `release`)
    #[arg(short, long)]
    pub mbid: Option<String>,
}

impl OverwriteCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let recording = if let Some(mbid) = self.mbid.as_ref() {
            MessyRecording::from_mbid_with_db(
                conn,
                &ALISTRAL_CLIENT.interzic,
                &read_mbid_from_input(mbid)
                    .expect_fatal("Couldn't read the mbid from the input. Check if it's correct")?,
            )
            .await
            .expect_fatal("Couldn't find this mbid. Are you sure it is correct?")?
        } else {
            MessyRecording {
                id: 0,
                title: self.recording.clone().ok_or_else(|| {
                    FatalError::new_string("Missing `recording` (`-r`) argument.")
                })?,
                artist_credits: self
                    .recording
                    .clone()
                    .ok_or_else(|| FatalError::new_string("Missing `artist` (`-a`) argument."))?,
                release: self.release.clone(),
                mbid: None,
            }
        };

        let recording = recording
            .upsert(&ALISTRAL_CLIENT.interzic.database_client)
            .await?;

        match self.target {
            InterzicMappingTarget::Youtube => {
                Youtube::save_ext_id(
                    &ALISTRAL_CLIENT.interzic,
                    Youtube::extract_id_from_text(&self.id).expect_fatal(
                        "Couldn't parse the youtube id from the input. Check if it's correct",
                    )?,
                    recording.id,
                    Some(self.user.to_string()),
                )
                .await?
            }
        };

        Ok(())
    }
}
