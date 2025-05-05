use musicbrainz_db_lite::MainEntity;
use musicbrainz_db_lite::Recording;
use regex::Regex;
use tuillez::formatter::FormatWithAsync;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::utils::formater;

pub struct DashETILint {
    recording: Recording,
    eti: String,
}

impl MbClippyLint for DashETILint {
    fn get_name() -> &'static str {
        "dash_eti"
    }

    async fn check(
        _client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(Vec::new());
        };

        // Check if the title has a common spotify "- ETI"
        let regex =
            Regex::new(r"(?i).+ - (original mix|sped up|slowed down|extra slowed down|(.+remix))$")
                .unwrap();

        let eti = match regex.captures(&recording.title) {
            None => return Ok(Vec::new()),
            Some(val) => val
                .get(0)
                .expect("The regex should return this capture group"),
        };

        Ok(vec![Self {
            recording: recording.clone(),
            eti: eti.as_str().to_string(),
        }])
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" seems to have ETI after a dash (-). This is often seen on spotify imports as spotify uses this style of ETI
    -> Convert the dash to parenthesis: `{} ({})`",
            self.recording.format_with_async(&formater(client)).await?,
                self.recording.title.clone().split(" - ").next().unwrap_or(""),
                self.eti
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
