use tuillez::formatter::FormatWithAsync;

use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::musicbrainz::label::Label;

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater<'_>> for Label {
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
                &self.name.truecolor(214, 0, 214).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/label/{}", &self.mbid),
        ))
    }
}
