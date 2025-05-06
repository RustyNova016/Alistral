use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Recording;
use musicbrainz_db_lite::models::musicbrainz::recording::relations::release::RecordingReleasesDBRel;
use musicbrainz_db_lite::models::shared_traits::db_relation::EntityURLDBRel;
use regex::Regex;
use tuillez::formatter::FormatWithAsync;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_result::LintResult;
use crate::clippy::lint_severity::LintSeverity;
use crate::utils::formater;
use crate::utils::is_release_harmony_compatible;

pub struct MissingRecordingLinksLint;

impl MbClippyLint for MissingRecordingLinksLint {
    type Result = MissingRecordingLinksLintRes;

    fn get_name() -> &'static str {
        "missing_recording_links"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<MissingRecordingLinksLintRes>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(Vec::new());
        };

        let recording_urls = recording
            .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
            .await?;

        let releases = recording
            .get_related_entity_or_fetch_as_task::<RecordingReleasesDBRel>(&client.mb_database)
            .await?;

        let mut lints = Vec::new();

        for release in releases {
            if !is_release_harmony_compatible(&client, &release).await? {
                continue;
            }

            let release_urls = release
                .get_related_entity_or_fetch_as_task::<EntityURLDBRel>(&client.mb_database)
                .await?;

            for release_url in release_urls {
                if !recording_urls.iter().any(|recording_url| {
                    check_url_parity(&release_url.ressource, &recording_url.ressource)
                }) {
                    lints.push(MissingRecordingLinksLintRes {
                        recording: recording.clone(),
                        link: release_url.ressource.clone(),
                    });
                }
            }
        }

        Ok(lints)
    }
}

/// Checks if an url on a release is matching an url on the recording.
fn check_url_parity(release_url: &str, recording_url: &str) -> bool {
    if same_start("https://open.spotify.com", release_url, recording_url) {
        return true;
    }

    if same_start("https://www.deezer.com", release_url, recording_url) {
        return true;
    }

    if same_start("https://music.apple.com", release_url, recording_url) {
        return true;
    }

    if same_start("https://tidal.com", release_url, recording_url) {
        return true;
    }

    if same_start("https://www.beatport.com", release_url, recording_url) {
        return true;
    }

    if same_start("https://www.youtube.com", release_url, recording_url)
        && release_url == recording_url
    {
        return true;
    }

    false
}

fn same_start(domain: &str, release_url: &str, recording_url: &str) -> bool {
    release_url.starts_with(domain) && recording_url.starts_with(domain)
}

pub struct MissingRecordingLinksLintRes {
    recording: Recording,
    link: String,
}

impl LintResult for MissingRecordingLinksLintRes {
    fn get_name() -> &'static str {
        MissingRecordingLinksLint::get_name()
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" is available at `{}`, but doesn't have a link on the Recording level.
    -> Add the link relationship to the recording",
            self.recording.format_with_async(&formater(client)).await?,
            self.link
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

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        Ok(vec![MbClippyLintHint::new("\"ETI\" means \"Extra title information\" and refers to information in the title of a track that isn't the name".to_string())])
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingRelation
    }
}

#[cfg(test)]
mod test {

    use musicbrainz_db_lite::Recording;

    use crate::SymphonyzeClient;
    use crate::clippy::lints::dash_eti::DashETILint;
    use crate::testing::should_trigger_lint;
    use crate::testing::shouldnt_trigger_lint;
    use crate::testing::test_name;

    #[tokio::test]
    async fn should_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("dash_eti.json").await;

        should_trigger_lint::<DashETILint, Recording>(
            &client,
            "857b3c6e-ae47-4365-94d4-49fcfdbfbaa4",
        )
        .await;
        should_trigger_lint::<DashETILint, Recording>(
            &client,
            "7972f99c-8015-4b37-972c-875b4fb421c6",
        )
        .await;
        should_trigger_lint::<DashETILint, Recording>(
            &client,
            "43bc29ef-6565-4fa3-a033-9e7d8f61f8e2",
        )
        .await;
        should_trigger_lint::<DashETILint, Recording>(
            &client,
            "3d1026b2-1cf6-40da-bd54-634d3c3799b1",
        )
        .await;
        should_trigger_lint::<DashETILint, Recording>(
            &client,
            "4e29b6c3-167d-4743-a749-198dd44b5691",
        )
        .await;
    }

    #[tokio::test]
    async fn shouldnt_trigger() {
        let client = SymphonyzeClient::get_testing_client(&test_name()).await;
        client.load_test_data("dash_eti.json").await;

        shouldnt_trigger_lint::<DashETILint, Recording>(
            &client,
            "b1319ac4-d7e1-4496-9db6-2a76c3221b88",
        )
        .await;
        shouldnt_trigger_lint::<DashETILint, Recording>(
            &client,
            "3711a2a2-7e39-4f18-b970-679d91f0794f",
        )
        .await;
    }
}
