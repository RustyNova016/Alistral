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
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        // Check if the title has a common spotify "- ETI"
        let regex =
            Regex::new(r"(?i).+ - (original mix|sped up|slowed down|extra slowed down|(.+remix))$")
                .unwrap();

        let eti = match regex.captures(&recording.title) {
            None => return Ok(None),
            Some(val) => val
                .get(0)
                .expect("The regex should return this capture group"),
        };

        Ok(Some(Self {
            recording: recording.clone(),
            eti: eti.as_str().to_string(),
        }))
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
