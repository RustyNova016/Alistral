use crate::models::musicbrainz::artist::Artist;

impl Artist {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.name.truecolor(20, 163, 249).to_string(),
                &Some(self.disambiguation.clone()),
            ),
            &self.get_url_link(listenbrainz),
        ))
    }

    #[cfg(feature = "pretty_format")]
    pub fn get_url_link(&self, listenbrainz: bool) -> String {
        if !listenbrainz {
            format!("https://musicbrainz.org/artist/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/artist/{}", &self.mbid)
        }
    }
}
