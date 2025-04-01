use inquire::InquireError;
use inquire::Text;
use interzic::models::services::youtube::Youtube;
use tuillez::inquire_ext::select_enum::select_enum;

use crate::tools::interzic::InterzicMappingTarget;
use crate::tools::interzic::overwrite::interactive::OverwriteError;

pub(super) fn get_target_id() -> Result<String, OverwriteError> {
    let target = prompt_target()?;
    prompt_id(target)
}

fn prompt_target() -> Result<InterzicMappingTarget, InquireError> {
    select_enum::<InterzicMappingTarget>("Overwrite the id on which service?").prompt()
}

fn prompt_id(target: InterzicMappingTarget) -> Result<String, OverwriteError> {
    let val = Text::new("The source recording id: ").prompt()?;

    match target {
        InterzicMappingTarget::Youtube => {
            Youtube::extract_id_from_text(&val).ok_or_else(|| OverwriteError::YoutubeIdParsing(val))
        }
    }
}
