use clap::Parser;
use interzic::models::services::musicbrainz::Musicbrainz;
use tuillez::fatal_error::IntoFatal;

use crate::api::clients::ALISTRAL_CLIENT;

#[derive(Parser, Debug, Clone)]
pub struct ReloadCommand;

impl ReloadCommand {
    pub async fn run(&self, _conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        Musicbrainz::reload_urls(&ALISTRAL_CLIENT.interzic)
            .await
            .expect_fatal("Couldn't reload URLs")?;

        Ok(())
    }
}
