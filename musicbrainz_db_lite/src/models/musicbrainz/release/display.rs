#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsync;

#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::release::Release;

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater<'_>> for Release {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater<'_>) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        let link = if !ft.listenbrainz_link {
            format!("https://musicbrainz.org/release/{}", &self.mbid)
        } else {
            format!("https://listenbrainz.org/release/{}", &self.mbid)
        };

        let name_format = hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(242, 244, 123).to_string(),
                &self.disambiguation,
            ),
            &link,
        );

        if ft.artist_credits {
            Ok(format!(
                "{} by {}",
                name_format,
                self.get_artist_credits_or_fetch(
                    ft.client.get_raw_connection().await?.as_mut(),
                    ft.client
                )
                .await?
                .format_with_async(ft)
                .await?
            ))
        } else {
            Ok(name_format)
        }
    }
}
