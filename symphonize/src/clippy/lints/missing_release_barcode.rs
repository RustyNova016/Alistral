use std::sync::LazyLock;

use musicbrainz_db_lite::HasUrls;
use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use tuillez::formatter::FormatWithAsync;
use url::Host;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::MusicbrainzLints;
use crate::utils::formater;

static DOMAINS_WITH_UPC: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "open.spotify.com",
        "www.deezer.com",
        "music.apple.com",
        "tidal.com",
        "www.beatport.com",
    ]
});

pub struct MissingBarcodeLint {
    release: Release,
    barcode_url: Option<String>,
}

impl MissingBarcodeLint {
    /// Check the links of the release to determine if it has a missing barcode
    async fn check_external_links(
        client: &SymphonyzeClient,
        release: &Release,
    ) -> Result<Option<Self>, crate::Error> {
        // Check if there is no barcode
        if release
            .barcode
            .as_ref()
            .is_some_and(|code| !code.is_empty())
        {
            return Ok(None);
        }

        for domain in DOMAINS_WITH_UPC.iter() {
            let host = Host::Domain(*domain);
            let urls = release
                .get_urls_with_host(&client.mb_database, &host)
                .await?;

            match urls.first() {
                None => continue,
                Some(url) => {
                    return Ok(Some(Self {
                        release: release.clone(),
                        barcode_url: Some(url.ressource.clone()),
                    }));
                }
            }
        }

        Ok(None)
    }

    /// Check if the barcode is empty, and not set as not having a barcode
    fn check_for_empty_and_not_set(release: &Release) -> Option<Self> {
        if release.barcode.as_ref().is_some() {
            return None;
        }

        Some(Self {
            release: release.clone(),
            barcode_url: None,
        })
    }
}

impl MbClippyLint for MissingBarcodeLint {
    fn get_name() -> &'static str {
        "missing_release_barcode"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Release(release) = entity else {
            return Ok(None);
        };

        if let Some(lint) = Self::check_external_links(client, release).await? {
            return Ok(Some(lint));
        }

        Ok(Self::check_for_empty_and_not_set(release))
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Release \"{}\" has no barcode
-> No barcode has been entered for this release, nor has been set has not having one.",
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
            name: "Release edit".to_string(),
            url: format!("https://musicbrainz.org/release/{}/edit", self.release.mbid),
        });

        if let Some(url) = &self.barcode_url {
            out.push(MbClippyLintLink {
                name: "Harmony search".to_string(),
                url: format!(
                    "https://harmony.pulsewidth.org.uk/release?url={url}&category=prefered"
                ),
            });
        }

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        if let Some(url) = &self.barcode_url {
            hints.push(MbClippyLintHint::new(format!(
                "Found a linked url that can provide a barcode: `{url}`"
            )));
        } else {
            hints.push(MbClippyLintHint::new("No barcode has been entered for this release, nor has been set has not having one.".to_string()));
        }

        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        if self.barcode_url.is_none() {
            LintSeverity::MissingData
        } else {
            LintSeverity::WrongData
        }
    }
}

impl From<MissingBarcodeLint> for MusicbrainzLints {
    fn from(value: MissingBarcodeLint) -> Self {
        Self::MissingBarcodeLint(value)
    }
}

#[cfg(test)]
mod test {
    use musicbrainz_db_lite::Release;

    use crate::SymphonyzeClient;
    use crate::clippy::lints::missing_release_barcode::MissingBarcodeLint;
    use crate::testing::should_trigger_lint;
    use crate::testing::test_name;

    #[tokio::test]
    async fn should_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("missing_release_barcode.json").await;

        should_trigger_lint::<MissingBarcodeLint, Release>(
            &client,
            "b241289a-bf04-4ed8-a7df-a911aea19554",
        )
        .await;
    }

    // #[tokio::test]
    // async fn shouldnt_trigger() {
    //     let client = SymphonyzeClient::get_testing_client(&test_name()).await;
    //     client.load_test_data("dash_eti.json").await;

    //     shouldnt_trigger_lint::<DashETILint, Recording>(
    //         &client,
    //         "b1319ac4-d7e1-4496-9db6-2a76c3221b88",
    //     )
    //     .await;
    //     shouldnt_trigger_lint::<DashETILint, Recording>(
    //         &client,
    //         "3711a2a2-7e39-4f18-b970-679d91f0794f",
    //     )
    //     .await;
    // }
}
