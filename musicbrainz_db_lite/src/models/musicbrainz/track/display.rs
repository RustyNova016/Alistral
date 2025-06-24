#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsync;

#[cfg(feature = "pretty_format")]
use crate::Track;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
#[cfg(feature = "pretty_format")]
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;

#[cfg(feature = "pretty_format")]
impl FormatWithAsync<MusicbrainzFormater> for Track {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;

        let name_format = self.title.truecolor(50, 254, 134).to_string();

        if ft.artist_credits {
            Ok(format!(
                "{} by {}",
                &name_format,
                self.get_related_entity::<ArtistCreditDBRel>(
                    &mut *ft.client.get_raw_connection().await?
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
