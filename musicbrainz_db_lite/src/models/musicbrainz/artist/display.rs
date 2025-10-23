#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsyncDyn;
#[cfg(feature = "pretty_format")]
use tuillez::reexports::async_trait;

#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::artist::Artist;

#[cfg(feature = "pretty_format")]
#[async_trait]
impl FormatWithAsyncDyn<MusicbrainzFormater> for Artist {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
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
