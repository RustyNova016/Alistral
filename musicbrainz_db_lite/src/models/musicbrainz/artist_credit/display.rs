use core::fmt::Display;
use core::fmt::Write;

use tuillez::formatter::FormatWithAsync;

use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::musicbrainz::artist_credit::ArtistCredits;

impl Display for ArtistCredits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.1 {
            write!(f, "{}", row)?;
        }
        Ok(())
    }
}

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater<'_>> for ArtistCredits {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater<'_>) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        let mut out = String::new();

        for credit in &self.1 {
            let link = if !ft.listenbrainz_link {
                format!("https://musicbrainz.org/artist/{}", &credit.artist_gid)
            } else {
                format!("https://listenbrainz.org/artist/{}", &credit.artist_gid)
            };

            write!(
                out,
                "{}{}",
                hyperlink_rename(&credit.name.truecolor(20, 163, 249), &link),
                credit.join_phrase
            )
            .expect("Display format is infaillible");
        }
        Ok(out)
    }
}
