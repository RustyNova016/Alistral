use inquire::Text;
use interzic::models::services::youtube::Youtube;
use tuillez::inquire_ext::select_enum::select_enum;

use crate::tools::interzic::IdOrigin;
use crate::utils::cli::read_mbid_from_input;
use crate::ALISTRAL_CLIENT;

pub(super) async fn get_source_id() -> Result<String, crate::Error> {
    println!("Enter the id of the recording to overwrite (Accept MBIDs and external ids, as well as URLs)");
    let id = Text::new("ID: ").prompt()?;

    println!();
    println!("Select the origin of the id");
    let origin = select_enum::<IdOrigin>("Origin:").prompt()?;

    match origin {
        IdOrigin::Musicbrainz => Ok(read_mbid_from_input(&id)),
        IdOrigin::Youtube => {}
    }

    todo!()
}


async fn prompt_mbid_from_youtube(yt_id: &str, user: Option<&str>) -> Result<Option<String>, crate::Error> {
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