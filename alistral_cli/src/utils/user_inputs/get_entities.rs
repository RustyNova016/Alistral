use musicbrainz_db_lite::GetOrFetch;
use musicbrainz_db_lite::Recording;

use crate::ALISTRAL_CLIENT;
use crate::utils::user_inputs::UserInputParser;

impl UserInputParser {
    /// Parse a recording from the input. End the program with no panics if the data is wrong
    pub async fn parse_recording(mbid: &str) -> Recording {
        let Some(mbid) = Self::read_mbid_from_input(mbid) else {
            tracing::error!("Couldn't read mbid from input");
            std::process::exit(2);
        };

        match Recording::get_or_fetch_as_task(ALISTRAL_CLIENT.musicbrainz_db.clone(), &mbid)
            .await
            .unwrap()
        {
            Some(val) => val,
            None => {
                tracing::error!("Couldn't find a recording with mbid {mbid}");
                std::process::exit(2);
            }
        }
    }
}
