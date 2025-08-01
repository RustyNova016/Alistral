use musicbrainz_db_lite::models::musicbrainz::recording::Recording;

use crate::ALISTRAL_CLIENT;

use super::clap_error;
use super::read_mbid_from_input;

pub async fn assert_recording_mbid(conn: &mut sqlx::SqliteConnection, id: &str) -> String {
    let Some(id) = read_mbid_from_input(id) else {
        clap_error(
            format!("Couldn't parse `{id}` as an mbid"),
            clap::error::ErrorKind::ValueValidation,
        )
    };

    match Recording::get_or_fetch(conn, &ALISTRAL_CLIENT.musicbrainz_db, &id).await {
        Ok(Some(_)) => id.to_string(),
        Ok(None) => clap_error(
            format!("MBID `{id}` couldn't be found."),
            clap::error::ErrorKind::ValueValidation,
        ),
        Err(_) => {
            panic!()
        }
    }
}