use clap::Parser;
use interzic::models::messy_recording::MessyRecording;
use interzic::models::services::musicbrainz::Musicbrainz;
use tuillez::fatal_error::IntoFatal;
use tuillez::fatal_error::OptIntoFatal as _;

use crate::ALISTRAL_CLIENT;
use crate::utils::cli::read_mbid_from_input;

/// Reload recording data from Musicbrainz
#[derive(Parser, Debug, Clone)]
pub struct ReloadCommand {
    /// Reload only this mbid
    mbid: Option<String>,
}

impl ReloadCommand {
    pub async fn run(&self) -> Result<(), crate::Error> {
        let conn = &mut *ALISTRAL_CLIENT.get_conn().await;

        if let Some(mbid) = &self.mbid {
            let mbid = &read_mbid_from_input(mbid)
                .expect_fatal("Couldn't read the mbid from the input. Check if it's correct")?;

            let recording =
                MessyRecording::from_mbid_with_db(conn, &ALISTRAL_CLIENT.interzic, mbid)
                    .await
                    .expect_fatal("Couldn't find the recording associated to the MBID")?
                    .upsert(&ALISTRAL_CLIENT.interzic.database_client)
                    .await?;

            Musicbrainz::fetch_and_save_urls(&ALISTRAL_CLIENT.interzic, &recording).await?;

            return Ok(());
        }

        Musicbrainz::reload_urls(&ALISTRAL_CLIENT.interzic)
            .await
            .expect_fatal("Couldn't reload URLs")?;

        Ok(())
    }
}
