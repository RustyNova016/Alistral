#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsync;

#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::musicbrainz::work::Work;

impl Work {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self) -> Result<String, crate::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 214).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/work/{}", &self.mbid),
        ))
    }
}

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater<'_>> for Work {
    type Error = crate::Error;

    async fn format_with_async(
        &self,
        _ft: &MusicbrainzFormater<'_>,
    ) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        Ok(hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 214).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/work/{}", &self.mbid),
        ))
    }
}
