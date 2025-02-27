use crate::models::musicbrainz::label::Label;

impl Label {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self) -> Result<String, crate::Error> {
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
