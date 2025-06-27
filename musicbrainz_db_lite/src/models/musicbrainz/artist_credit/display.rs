use core::fmt::Display;
#[cfg(feature = "pretty_format")]
use core::fmt::Write;

#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsync;

#[cfg(feature = "pretty_format")]
use crate::ArtistCredit;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::musicbrainz::artist_credit::ArtistCredits;

impl Display for ArtistCredits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.1 {
            write!(f, "{row}")?;
        }
        Ok(())
    }
}

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater> for Vec<ArtistCredit> {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        let mut out = String::new();

        for credit in self {
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

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater> for ArtistCredits {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        self.1.format_with_async(ft).await
    }
}
