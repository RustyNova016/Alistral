use std::sync::LazyLock;

use format_url::FormatUrl;
use musicbrainz_db_lite::Release;
use musicbrainz_db_lite::Url;
use musicbrainz_db_lite::models::musicbrainz::recording::relations::releases::RecordingReleasesDBRel;
use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};
use musicbrainz_db_lite::models::shared_traits::db_relation::EntityURLDBRel;
use tuillez::formatter::FormatWithAsync;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

use crate::SymphonyzeClient;
use crate::utils::formater;
use crate::utils::link_supported_by_harmony;

static LINK_DOMAINS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "https://open.spotify.com",
        "https://www.deezer.com",
        "https://music.apple.com",
        "https://tidal.com",
        "https://www.beatport.com",
        "https://www.youtube.com",
    ]
});

pub struct MissingRecordingLink {
    recording: Recording,
    parent_release: Release,
    link_missing: String,
}

impl MissingRecordingLink {
    async fn check_release(
        client: &SymphonyzeClient,
        release: &Release,
        recording: &Recording,
        recording_urls: &[Url],
    ) -> Result<Option<Self>, crate::Error> {
        let release_urls = release
            .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
            .await?;

        for domain in LINK_DOMAINS.iter() {
            let release_link = match Self::get_link_with_domain(domain, &release_urls) {
                None => continue,
                Some(url) => url,
            };

            if Self::get_link_with_domain(domain, recording_urls).is_none() {
                return Ok(Some(Self {
                    recording: recording.clone(),
                    parent_release: release.clone(),
                    link_missing: release_link.ressource.to_owned(),
                }));
            }
        }

        Ok(None)
    }

    fn get_link_with_domain<'a>(domain: &str, urls: &'a [Url]) -> Option<&'a Url> {
        urls.iter().find(|url| url.ressource.starts_with(domain))
    }
}

impl MbClippyLint for MissingRecordingLink {
    fn get_name() -> &'static str {
        "missing_recording_link"
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

        let releases = recording
            .get_related_entity_or_fetch_as_task::<RecordingReleasesDBRel>(&client.mb_database)
            .await?;

        for release in releases {
            if let Some(lint) =
                Self::check_release(client, &release, recording, &recording_urls).await?
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
            "Recording \"{}\" has a release with links that could be set on the recording too",
            self.recording.format_with_async(&formater(client)).await?
        ))
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error> {
        let mut out = Vec::new();

        out.push(MbClippyLintLink {
            name: "Recording".to_string(),
            url: format!("https://musicbrainz.org/recording/{}", self.recording.mbid),
        });

        out.push(MbClippyLintLink {
            name: "Recording editing".to_string(),
            url: format!(
                "https://musicbrainz.org/recording/{}/edit",
                self.recording.mbid
            ),
        });

        if link_supported_by_harmony(&self.link_missing) {
            out.push(MbClippyLintLink {
                name: "Harmony release actions".to_string(),
                url: format!(
                    "https://harmony.pulsewidth.org.uk/release/actions?release_mbid={}",
                    self.parent_release.mbid
                ),
            });
        } else if self.link_missing.starts_with("https://www.youtube.com") {
            let url = FormatUrl::new("https://musicbrainz.org")
                .with_path_template("/recording/:MBID/edit")
                .with_substitutes(vec![("MBID", &self.recording.mbid)])
                .with_query_params(vec![
                    ("edit-recording.url.0.text", &self.link_missing),
                    ("edit-recording.url.0.link_type_id", "268"),
                    ("edit-recording.edit_note", &format!(
                        "Link copied from release https://musicbrainz.org/release/`{}`. Found by Alistral lint `{}`",
                         self.parent_release.mbid,
                         Self::get_name()
                    ))
                ]).format_url();

            out.push(MbClippyLintLink {
                name: "Add to the recording".to_string(),
                url,
            });
        }

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        hints.push(MbClippyLintHint::new(format!(
            "Found url `{}` on the release missing an equivalent on the recording",
            self.link_missing
        )));

        // TODO: #526 missing_recording_work: add hint that remixes uses the same work
        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingRelation
    }
}
