use inquire::InquireError;
use inquire::Text;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::youtube::Youtube;
use tuillez::inquire_ext::select_enum::select_enum;
use tuillez::utils::hyperlink_rename;

use crate::tools::interzic::InterzicMappingTarget;
use crate::tools::interzic::overwrite::interactive::OverwriteError;

pub(super) fn get_target_id(
    recording: &MessyRecording,
) -> Result<(String, InterzicMappingTarget), OverwriteError> {
    let target = prompt_target()?;
    Ok((prompt_id(target.clone(), recording)?, target))
}

fn prompt_target() -> Result<InterzicMappingTarget, InquireError> {
    select_enum::<InterzicMappingTarget>("Overwrite the id on which service?").prompt()
}

fn prompt_id(
    target: InterzicMappingTarget,
    recording: &MessyRecording,
) -> Result<String, OverwriteError> {
    match target {
        InterzicMappingTarget::Youtube => {
            println!("Please enter the video ID for the new mapping (Allows urls)");
            println!();
            println!(
                "{}",
                hyperlink_rename(
                    &"Youtube search results",
                    &format!("https://www.youtube.com/results?search_query={}", recording)
                )
            );
            println!();
        }
    }

    let val = Text::new("Target recording id: ").prompt()?;

    match target {
        InterzicMappingTarget::Youtube => {
            Youtube::extract_id_from_text(&val).ok_or_else(|| OverwriteError::YoutubeIdParsing(val))
        }
    }
}
