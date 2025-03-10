use crate::models::musicbrainz::recording::Recording;
#[cfg(feature = "pretty_format")]
use crate::DBClient;

impl Recording {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self) -> Result<String, crate::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 114).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/recording/{}", &self.mbid),
        ))
    }

    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
        listenbrainz: bool,
    ) -> Result<String, crate::Error> {
        Ok(format!(
            "{} by {}",
            self.pretty_format().await?,
            self.get_artist_credits_or_fetch(conn, client)
                .await?
                .pretty_format(listenbrainz)
                .await?
        ))
    }
}
