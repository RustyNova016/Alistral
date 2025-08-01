use clap::Parser;
use interzic::models::services::youtube::Youtube;
use tuillez::fatal_error::OptIntoFatal;

use crate::ALISTRAL_CLIENT;
use crate::tools::interzic::get_mapping::InterzicMappingTarget;

#[derive(Parser, Debug, Clone)]
pub struct ReverseMappingCommand {
    /// Get the mapping of which service?
    pub source: InterzicMappingTarget,

    /// The id on the external service
    pub id: String,

    /// Search the mapping of this user
    pub user: Option<String>,
}

impl ReverseMappingCommand {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        let recordings = match self.source {
            InterzicMappingTarget::Youtube => {
                Youtube::get_recordings_from_id(
                    &ALISTRAL_CLIENT.interzic,
                    &Youtube::extract_id_from_text(&self.id).expect_fatal(
                        "Couldn't parse the youtube id from the input. Check if it's correct",
                    )?,
                    self.user.as_deref(),
                )
                .await?
            }
        };

        if recordings.is_empty() {
            println!("Found no recordings");
            return Ok(());
        }

        println!("Found recording(s)");
        for rec in recordings {
            println!();
            println!("Title: {}", rec.title);
            println!("Artist credit: {}", rec.artist_credits);
            if let Some(release) = rec.release {
                println!("Release: {release}");
            }
            if let Some(mbid) = rec.mbid {
                println!("MBID: {mbid}");
            }
        }

        Ok(())
    }
}
