use std::sync::LazyLock;

use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::Url;
use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};
use musicbrainz_db_lite::models::shared_traits::db_relation::ArtistFromCreditsRelation;
use musicbrainz_db_lite::models::shared_traits::db_relation::EntityURLDBRel;
use tuillez::formatter::FormatWithAsync;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

use crate::SymphonyzeClient;
use crate::utils::formater;
use crate::utils::harmony::get_harmony_compatible_release_for_recording;

static LINK_DOMAINS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "https://open.spotify.com",
        "https://www.deezer.com",
        "https://music.apple.com",
        "https://tidal.com",
        "https://www.beatport.com",
        //"https://www.youtube.com", Youtube links may not be from the artist channel's
    ]
});

pub struct MissingArtistLink {
    recording: Recording,
    artist: Artist,
    link_missing: String,
}

impl MissingArtistLink {
    async fn check_artist(
        client: &SymphonyzeClient,
        artist: &Artist,
        recording: &Recording,
        recording_urls: &[Url],
    ) -> Result<Option<Self>, crate::Error> {
        let artist_urls = artist
            .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
            .await?;

        for domain in LINK_DOMAINS.iter() {
            let recording_link = match Self::get_link_with_domain(domain, recording_urls) {
                None => continue,
                Some(url) => url,
            };

            if Self::get_link_with_domain(domain, &artist_urls).is_none() {
                return Ok(Some(Self {
                    recording: recording.clone(),
                    artist: artist.clone(),
                    link_missing: recording_link.ressource.to_owned(),
                }));
            }
        }

        Ok(None)
    }

    fn get_link_with_domain<'a>(domain: &str, urls: &'a [Url]) -> Option<&'a Url> {
        urls.iter().find(|url| url.ressource.starts_with(domain))
    }
}

impl MbClippyLint for MissingArtistLink {
    fn get_name() -> &'static str {
        "missing_artist_link"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        // Check if a release of the recording got a link that can be moved to the recording level
        // Whether by direct copy or harmony

        let recording_urls = recording
            .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
            .await?;

        let artists = recording
            .get_related_entity_or_fetch_as_task::<ArtistFromCreditsRelation>(&client.mb_database)
            .await?;

        for artist in artists {
            if let Some(lint) =
                Self::check_artist(client, &artist, recording, &recording_urls).await?
            {
                return Ok(Some(lint));
            }
        }

        Ok(None)
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" has an URL that imply that an artist page for {} exist on the same website. However no corresponding URL is found on the artist",
            self.recording.format_with_async(&formater(client)).await?,
            self.artist.format_with_async(&formater(client)).await?
        ))
    }

    async fn get_links(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Recording".to_string(),
            url: format!("https://musicbrainz.org/artist/{}", self.artist.mbid),
        });

        out.push(MbClippyLintLink {
            name: "Artist editing".to_string(),
            url: format!("https://musicbrainz.org/artist/{}/edit", self.artist.mbid),
        });

        if let Some(release) =
            get_harmony_compatible_release_for_recording(client, &self.recording).await?
        {
            out.push(MbClippyLintLink {
                name: "Harmony release actions".to_string(),
                url: format!(
                    "https://harmony.pulsewidth.org.uk/release/actions?release_mbid={}",
                    release.mbid
                ),
            });
        }

        Ok(out)
    }

    async fn get_hints(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        hints.push(MbClippyLintHint::new(format!(
            "Found url `{}` on the recording, but {} has no linked artist page for the same website",
            self.link_missing,
            self.artist.format_with_async(&formater(client)).await?
        )));

        // TODO: #526 missing_recording_work: add hint that remixes uses the same work
        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingRelation
    }
}

#[cfg(test)]
mod test {

    use musicbrainz_db_lite::Recording;

    use crate::SymphonyzeClient;
    use crate::clippy::lints::missing_artist_link::MissingArtistLink;
    use crate::testing::should_trigger_lint;
    use crate::testing::shouldnt_trigger_lint;
    use crate::testing::test_name;

    #[tokio::test]
    async fn should_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("missing_artist_links.json").await;

        should_trigger_lint::<MissingArtistLink, Recording>(
            &client,
            "953d57c1-06d4-4faa-b7b7-91f09912ff99",
        )
        .await;
    }

    #[tokio::test]
    async fn shouldnt_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("missing_artist_links.json").await;

        shouldnt_trigger_lint::<MissingArtistLink, Recording>(
            &client,
            "10abef42-4627-453a-90ae-eed80841c198",
        )
        .await;
    }
}
