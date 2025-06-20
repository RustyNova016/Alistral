use std::sync::LazyLock;

use musicbrainz_db_lite::HasUrls as _;
use musicbrainz_db_lite::Release;
use musicbrainz_db_lite::models::musicbrainz::recording::relations::releases::RecordingReleasesDBRel;
use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};
use musicbrainz_db_lite::models::shared_traits::db_relation::RecordingISRCDBRel;
use tuillez::formatter::FormatWithAsync;
use url::Host;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::links::create_harmony_action_link;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::utils::formater;

static DOMAINS_WITH_ISRCS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "open.spotify.com",
        "www.deezer.com",
        "tidal.com",
        "www.beatport.com",
    ]
});

pub struct MissingISRCLint {
    recording: Recording,
    release_with_isrc: Release,
}

impl MissingISRCLint {
    async fn check_release(
        client: &SymphonyzeClient,
        release: &Release,
        recording: &Recording,
    ) -> Result<Option<Self>, crate::Error> {
        for domain in DOMAINS_WITH_ISRCS.iter() {
            let host = Host::Domain(*domain);
            if release
                .has_url_with_host(&client.mb_database, &host)
                .await?
                .is_some()
            {
                return Ok(Some(Self {
                    recording: recording.clone(),
                    release_with_isrc: release.clone(),
                }));
            }
        }

        Ok(None)
    }
}

impl MbClippyLint for MissingISRCLint {
    fn get_name() -> &'static str {
        "missing_isrc"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        let isrcs = recording
            .get_related_entity_or_fetch_as_task::<RecordingISRCDBRel>(&client.mb_database)
            .await?;

        if !isrcs.is_empty() {
            return Ok(None);
        }

        let releases = recording
            .get_related_entity_or_fetch_as_task::<RecordingReleasesDBRel>(&client.mb_database)
            .await?;

        for release in releases {
            if let Some(lint) = Self::check_release(client, &release, recording).await? {
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
            "Recording \"{}\" has no ISRCs, but got a release that provide it",
            self.recording.format_with_async(&formater(client)).await?
        ))
    }

    async fn get_links(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Recording editing".to_string(),
            url: format!(
                "https://musicbrainz.org/recording/{}/edit",
                self.recording.mbid
            ),
        });

        if let Some(link) =
            create_harmony_action_link(client, MainEntity::Release(self.release_with_isrc.clone()))
                .await?
        {
            out.push(link);
        }

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let hints = Vec::new();

        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingRelation
    }
}
