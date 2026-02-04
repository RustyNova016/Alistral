#[cfg(feature = "pretty_format")]
use chrono::Duration;
#[cfg(feature = "pretty_format")]
use tuillez::extensions::chrono_exts::DurationExt;
#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsyncDyn;
#[cfg(feature = "pretty_format")]
use tuillez::reexports::async_trait;

#[cfg(feature = "pretty_format")]
use crate::Track;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
#[cfg(feature = "pretty_format")]
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;

#[cfg(feature = "pretty_format")]
#[async_trait]
impl FormatWithAsyncDyn<MusicbrainzFormater> for Track {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;

        let name_format = self.title.truecolor(50, 254, 134).to_string();

        let credited_format = if ft.artist_credits {
            format!(
                "{} by {}",
                &name_format,
                self.get_related_entity::<ArtistCreditDBRel>(
                    &mut *ft.client.get_raw_connection().await?
                )
                .await?
                .format_with_async(ft)
                .await?
            )
        } else {
            name_format
        };

        if ft.duration && self.length.is_some() {
            Ok(format!(
                "{} {}",
                credited_format,
                format!(
                    "({})",
                    Duration::seconds(self.length.unwrap() / 1000)
                        .to_humantime()
                        .unwrap()
                )
                .truecolor(100, 100, 100),
            ))
        } else {
            Ok(credited_format)
        }
    }
}
