#[cfg(feature = "pretty_format")]
use chrono::Duration;
#[cfg(feature = "pretty_format")]
use tuillez::extensions::chrono_exts::DurationExt;
#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsyncDyn;
#[cfg(feature = "pretty_format")]
use tuillez::reexports::async_trait;

#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::recording::Recording;
#[cfg(feature = "pretty_format")]
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;

#[cfg(feature = "pretty_format")]
#[async_trait]
impl FormatWithAsyncDyn<MusicbrainzFormater> for Recording {
    type Error = crate::Error;

    async fn format_with_async(&self, ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        use owo_colors::OwoColorize as _;
        use tuillez::utils::hyperlink_rename;

        use crate::utils::display::format_disambiguation;

        let name_format = hyperlink_rename(
            &format_disambiguation(
                &self.title.truecolor(0, 214, 114).to_string(),
                &self.disambiguation,
            ),
            &format!("https://musicbrainz.org/recording/{}", &self.mbid),
        );

        let title_with_maybe_artist_format = if ft.artist_credits {
            format!(
                "{} by {}",
                name_format,
                self.get_related_entity_or_fetch_as_task::<ArtistCreditDBRel>(&ft.client)
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
                title_with_maybe_artist_format,
                format!(
                    "({})",
                    Duration::seconds(self.length.unwrap() / 1000)
                        .to_humantime()
                        .unwrap()
                )
                .truecolor(100, 100, 100),
            ))
        } else {
            Ok(title_with_maybe_artist_format)
        }
    }
}
