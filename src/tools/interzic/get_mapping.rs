use clap::Parser;
use clap::ValueEnum;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::youtube::Youtube;
use tuillez::fatal_error::FatalError;
use tuillez::fatal_error::IntoFatal;
use tuillez::fatal_error::OptIntoFatal;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli::read_mbid_from_input;

#[derive(Parser, Debug, Clone)]
pub struct GetMappingCommand {
    /// Get the mapping of which service?
    pub target: InterzicMappingTarget,

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

    /// Search the mapping of this user
    #[arg(short, long)]
    pub user: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum InterzicMappingTarget {
    Youtube,
}

impl GetMappingCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        // Prioritise mbid resolving
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
                let id =
                    Youtube::get_or_query(&ALISTRAL_CLIENT.interzic, &recording, self.user.clone())
                        .await?;

                match id {
                    None => println!("Couldn't find a mapping for the recording"),
                    Some(id) => println!("Mapped to video: https://youtu.be/{}", id),
                }
            }
        }

        Ok(())
    }
}
