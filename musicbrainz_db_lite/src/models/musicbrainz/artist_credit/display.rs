use core::fmt::Display;
use core::fmt::Write;

use crate::models::musicbrainz::artist_credit::ArtistCredits;

impl Display for ArtistCredits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.1 {
            write!(f, "{}", row)?;
        }
        Ok(())
    }
}

impl ArtistCredits {
    #[cfg(feature = "pretty_format")]
    pub async fn pretty_format(&self, listenbrainz: bool) -> Result<String, crate::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        let mut out = String::new();

        for credit in &self.1 {
            let link = if !listenbrainz {
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
