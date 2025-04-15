use inquire::Text;
use interzic::models::services::youtube::Youtube;
use tuillez::inquire_ext::select_associated::select_associated;
use tuillez::inquire_ext::select_enum::select_enum;

use crate::ALISTRAL_CLIENT;
use crate::tools::interzic::IdOrigin;
use crate::utils::cli::read_mbid_from_input;

pub(super) async fn get_source_id(user: Option<&str>) -> Result<Option<String>, crate::Error> {
    println!(
        "Enter the id of the recording to overwrite (Accept MBIDs and external ids, as well as URLs)"
    );
    let id = Text::new("ID: ").prompt()?;

   
    let origin = select_enum::<IdOrigin>("Origin:").prompt()?;

    match origin {
        IdOrigin::Musicbrainz => Ok(read_mbid_from_input(&id)),
        IdOrigin::Youtube => Ok(prompt_mbid_from_youtube(&id, user).await?), //TODO: Error checking
    }
}

async fn prompt_mbid_from_youtube(
    yt_id: &str,
    user: Option<&str>,
) -> Result<Option<String>, crate::Error> {
    let recordings = Youtube::get_recordings_from_id(
        &ALISTRAL_CLIENT.interzic,
        &Youtube::extract_id_from_text_or_error(&yt_id)?,
        user,
    )
    .await?;

    if recordings.is_empty() {
        println!("Found no recordings");
        return Ok(None);
    }

    Ok(Some(
        select_associated("Which mapped recording?", recordings, |rec| {
            format!("{} - {}", rec.artist_credits, rec.title)
        })
        .prompt()?
        .into_data()
        .mbid
        .expect("The recording should have an MBID"),
    ))
}
