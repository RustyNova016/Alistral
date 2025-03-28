pub mod find_source;
use clap::Id;
use clap::Parser;
use inquire::InquireError;
use inquire::Select;
use inquire::Text;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::youtube::Youtube;
use itertools::Itertools;
use tuillez::fatal_error::IntoFatal;
use tuillez::fatal_error::OptIntoFatal as _;
use tuillez::inquire_ext::select_enum::select_enum;

use crate::ALISTRAL_CLIENT;
use crate::models::config::Config;
use crate::tools::interzic::IdOrigin;
use crate::tools::interzic::InterzicMappingTarget;
use crate::utils::cli::read_mbid_from_input;

#[derive(Debug, Default, Parser, Clone)]
pub(crate) struct InteractiveOverwriteCommand {}

impl InteractiveOverwriteCommand {
    pub async fn run(&self) -> Result<(), crate::Error> {
        let mut state = State::default();
        Ok(state.run().await?)
    }
}

#[derive(Debug, Default, Clone)]
struct State {
    selected_source: Option<IdOrigin>,
    selected_id: Option<String>,
    selected_mbid: Option<String>,
    selected_target: Option<InterzicMappingTarget>,
    user: Option<String>,
}

impl State {
    pub async fn run(&mut self) -> Result<(), crate::Error> {
        loop {
            if self.selected_id.is_none() {
                self.prompt_source_id()?;
            } else if self.selected_source.is_none() {
                self.prompt_source()?;
            } else if self.selected_mbid.is_none() {
                self.prompt_mbid()?;
            } else if self.selected_target.is_none() {
                self.prompt_target()?;
            } else {
                let id = self.prompt_new_id()?;

                match self.selected_target {
                    None => panic!("Target is not initialized"),
                    Some(InterzicMappingTarget::Youtube) => {
                        Youtube::save_ext_id(
                            &ALISTRAL_CLIENT.interzic,
                            Youtube::extract_id_from_text(&self.id).expect_fatal(
                                "Couldn't parse the youtube id. Check if it's correct",
                            )?,
                            self.selected_mbid.expect("No selected MBID"),
                            self.user,
                        )
                        .await?
                    }
                };
            }
        }
    }

    pub fn prompt_source(&mut self) -> Result<(), InquireError> {
        let val = select_enum::<IdOrigin>("From which service to find the source ID?").prompt()?;
        let _ = self.selected_source.insert(val);
        Ok(())
    }

    pub fn prompt_target(&mut self) -> Result<(), InquireError> {
        let val =
            select_enum::<InterzicMappingTarget>("Overwrite the id on which service?").prompt()?;
        let _ = self.selected_target.insert(val);
        Ok(())
    }

    pub async fn prompt_mbid(&mut self) -> Result<(), crate::Error> {
        if self
            .selected_source
            .as_ref()
            .is_some_and(|s| s == &IdOrigin::Musicbrainz)
        {
            self.selected_mbid = self.selected_id.clone();

            return Ok(());
        }

        MessyRecording::from_mbid_with_db(
            &mut *ALISTRAL_CLIENT.musicbrainz_db.get_raw_connection().await?,
            &ALISTRAL_CLIENT.interzic,
            &read_mbid_from_input(&self.selected_id.clone().expect("The source id isn't set"))
                .expect_fatal("Couldn't read the mbid from the input. Check if it's correct")?,
        )
        .await
        .expect_fatal("Couldn't find this mbid. Are you sure it is correct?")?;

        Ok(())
    }

    pub fn prompt_source_id(&mut self) -> Result<(), InquireError> {
        let val = Text::new("The source recording id: ").prompt()?;
        let _ = self.selected_id.insert(val);
        Ok(())
    }

    pub fn prompt_user(&mut self) -> Result<(), InquireError> {
        let val = Text::new("Overwrite for this user: ")
            .with_default(&Config::get_default_user().unwrap_or(String::new()))
            .prompt()?;
        let _ = self.selected_id.insert(val);
        Ok(())
    }

    pub fn prompt_new_id(&mut self) -> Result<String, InquireError> {
        Text::new("Overwrite with: ").prompt()
    }
}

async fn prompt_mbid_from_youtube(yt_id: &str, user: Option<&str>) -> Result<Option<String>, crate::Error> {
    let recordings = Youtube::get_recordings_from_id(
        &ALISTRAL_CLIENT.interzic,
        &Youtube::extract_id_from_text(&yt_id)
            .expect_fatal("Couldn't parse the youtube id from the input. Check if it's correct")?,
        user,
    )
    .await?;

    if recordings.is_empty() {
        println!("Found no recordings");
        return Ok(None);
    }

    let recording_titles = recordings.into_iter().map(|rec| format!("{} - {}", rec.artist_credits, rec.title)).collect_vec();

    let res = Select::new("Select recording: ", recording_titles).w

    println!("Found recording(s)");
    for rec in recordings {
        println!();
        println!("Title: {}", rec.title);
        println!("Artist credit: {}", rec.artist_credits);
        if let Some(release) = rec.release {
            println!("Release: {}", release);
        }
        if let Some(mbid) = rec.mbid {
            println!("MBID: {}", mbid);
        }
    }

    Ok(())
}
