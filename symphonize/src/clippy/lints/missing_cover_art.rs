use std::sync::LazyLock;

use ::url::Host;
use musicbrainz_db_lite::HasUrls as _;
use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Release;
use tuillez::formatter::FormatWithAsync;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

use crate::SymphonyzeClient;
use crate::utils::formater;

static DOMAINS_WITH_ARTWORK: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "https://open.spotify.com",
        "https://www.deezer.com",
        "https://music.apple.com",
        "https://tidal.com",
        "https://www.beatport.com",
        "https://www.youtube.com",
    ]
});

pub struct MissingWorkLint {
    release: Release,
}

impl MbClippyLint for MissingWorkLint {
    fn get_name() -> &'static str {
        "missing_cover_art"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Release(release) = entity else {
            return Ok(None);
        };

        if release.cover_count.is_some_and(|count| count > 1) {
            return Ok(None);
        }

        let urls = release.get_entity_urls(&client.mb_database).await?;

        if urls.into_iter().any(|url| {
            DOMAINS_WITH_ARTWORK
                .iter()
                .any(|host| url.match_host(&Host::Domain(host)))
        }) {
            Ok(Some(Self {
                release: release.clone(),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Release \"{}\" has no cover art, but is present on music providers that usually have it",
            self.release.format_with_async(&formater(client)).await?
        ))
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Release".to_string(),
            url: format!("https://musicbrainz.org/release/{}", self.release.mbid),
        });

        out.push(MbClippyLintLink {
            name: "Harmony release actions".to_string(),
            url: format!(
                "https://harmony.pulsewidth.org.uk/release/actions?release_mbid={}",
                self.release.mbid
            ),
        });

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        hints.push(MbClippyLintHint::new("If multiple providers have different cover art, the releases should be separated (Youtube thumbnails doesn't usually count)".to_string()));

        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingData
    }
}

// #[cfg(test)]
// mod test {

//     use musicbrainz_db_lite::Recording;

//     use crate::SymphonyzeClient;
//     use crate::clippy::lints::missing_work::MissingWorkLint;
//     use crate::testing::should_trigger_lint;
//     use crate::testing::shouldnt_trigger_lint;
//     use crate::testing::test_name;

//     #[tokio::test]
//     async fn should_trigger() {
//         let client = SymphonyzeClient::get_testing_client(&test_name()).await;
//         client.load_test_data("missing_work.json").await;

//         should_trigger_lint::<MissingWorkLint, Recording>(
//             &client,
//             "51feae17-26bd-4aa9-bfaa-ef408df58293",
//         )
//         .await;
//     }

//     #[tokio::test]
//     async fn shouldnt_trigger() {
//         let client = SymphonyzeClient::get_testing_client(&test_name()).await;
//         client.load_test_data("missing_work.json").await;

//         shouldnt_trigger_lint::<MissingWorkLint, Recording>(
//             &client,
//             "e8454ea9-b850-46b0-b8d8-22d024095819",
//         )
//         .await;
//     }
// }
