pub mod find_source;
pub mod get_target;
use clap::Id;
use clap::Parser;
use inquire::InquireError;
use inquire::Select;
use inquire::Text;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::youtube::Youtube;
use itertools::Itertools;
use strum::Display;
use strum::EnumIter;
use thiserror::Error;
use tracing::error;
use tuillez::fatal_error::IntoFatal;
use tuillez::fatal_error::OptIntoFatal as _;
use tuillez::inquire_ext::select_enum::select_enum;

use crate::ALISTRAL_CLIENT;
use crate::models::config::Config;
use crate::tools::interzic::IdOrigin;
use crate::tools::interzic::InterzicMappingTarget;
use crate::tools::interzic::overwrite::interactive::find_source::get_source_id;
use crate::tools::interzic::overwrite::interactive::get_target::get_target_id;
use crate::utils::cli::read_mbid_from_input;

#[derive(Debug, Default, Parser, Clone)]
pub(crate) struct InteractiveOverwriteCommand {}

impl InteractiveOverwriteCommand {
    pub async fn run(&self) -> Result<(), crate::Error> {
        let conn = &mut *ALISTRAL_CLIENT.musicbrainz_db.get_raw_connection().await?;
        let user = Self::get_user()?;

        loop {
            let Some(source) = get_source_id(Some(&user)).await? else {
                continue;
            };
            println!();

            // Create a messy recording from the mbid
            let recording =
                MessyRecording::from_mbid_with_db(conn, &ALISTRAL_CLIENT.interzic, &source)
                    .await
                    .expect_fatal("Couldn't find this mbid. Are you sure it is correct?")?;

            let recording = recording
                .upsert(&ALISTRAL_CLIENT.interzic.database_client)
                .await?;

            let target = get_target_id(&recording)?;

            match target.1 {
                InterzicMappingTarget::Youtube => {
                    Youtube::save_ext_id(
                        &ALISTRAL_CLIENT.interzic,
                        Youtube::extract_id_from_text(&target.0).expect_fatal(
                            "Couldn't parse the youtube id from the input. Check if it's correct",
                        )?,
                        recording.id,
                        Some(user.clone()),
                    )
                    .await?
                }
            };

            println!();
            if !Self::ask_continue()? {
                break;
            }
            println!();
        }

        Ok(())
    }

    fn get_user() -> Result<String, OverwriteError> {
        loop {
            let val = Text::new("Username (leave blank for default): ").prompt()?;
            if val.is_empty() {
                match Config::get_default_user() {
                    Some(val) => return Ok(val),
                    None => error!("No default user has been set. Please type your username"),
                }
            } else {
                return Ok(val)
            }
        }
    }

    fn ask_continue() -> Result<bool, OverwriteError> {
        match select_enum::<ContinuePrompt>("Continue? ").prompt()? {
            ContinuePrompt::Yes => Ok(true),
            ContinuePrompt::No => Ok(false),
        }
    }
}

#[derive(Debug, Error)]
pub enum OverwriteError {
    #[error("Couldn't parse a youtube ID from `{0}`")]
    YoutubeIdParsing(String),

    #[error(transparent)]
    Inquire(#[from] InquireError),
}

#[derive(Clone, Debug, EnumIter, Display, PartialEq, Eq)]
enum ContinuePrompt {
    Yes,
    No,
}
