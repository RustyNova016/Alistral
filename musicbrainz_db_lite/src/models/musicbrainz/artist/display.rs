use tuillez::formatter::FormatWithAsync;

use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::musicbrainz::artist::Artist;

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater<'_>> for Artist {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater<'_>) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        let link = if !ft.listenbrainz_link {
            format!("https://musicbrainz.org/artist/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/artist/{}", &self.mbid)
        };

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.name.truecolor(20, 163, 249).to_string(),
                &Some(self.disambiguation.clone()),
            ),
            &link,
        ))
    }
}
