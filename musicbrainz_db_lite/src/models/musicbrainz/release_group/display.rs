use crate::models::musicbrainz::release_group::ReleaseGroup;
#[cfg(feature = "pretty_format")]
use crate::DBClient;

impl ReleaseGroup {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(254, 173, 75).to_string(),
                &Some(self.disambiguation.clone()),
            ),
            &self.get_url_link(listenbrainz),
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
            self.pretty_format(listenbrainz).await?,
            self.get_artist_credits_or_fetch(conn, client)
                .await?
                .pretty_format(listenbrainz)
                .await?
        ))
    }

    #[cfg(feature = "pretty_format")]
    pub fn get_url_link(&self, listenbrainz: bool) -> String {
        if !listenbrainz {
            format!("https://musicbrainz.org/release-group/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/album/{}", &self.mbid)
        }
    }
}
