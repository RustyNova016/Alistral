use clap::Parser;
use clap::ValueEnum;
use interzic::models::messy_recording::MessyRecording;
use tuillez::fatal_error::FatalError;

use crate::api::clients::ALISTRAL_CLIENT;

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
    #[arg(short='g', long)]
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
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), FatalError>  {
        // Prioritise mbid resolving
        let recording = if let Some(mbid) = self.mbid.as_ref() {
            MessyRecording::from_mbid_with_db(conn, &ALISTRAL_CLIENT.interzic, mbid).await.expect("Error while creating recording")
        } else {
            MessyRecording {
                id: 0,
                title: self.recording.clone().ok_or_else(|| FatalError::new_string("Missing `recording` (`-r`) argument."))?,
                artist_credits: self.recording.clone().ok_or_else(|| FatalError::new_string("Missing `artist` (`-a`) argument."))?,
                release: self.release.clone(),
                mbid: None
            }
        };

        Ok(())
    }
}