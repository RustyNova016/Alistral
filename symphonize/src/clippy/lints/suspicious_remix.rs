use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use regex::Regex;
use tuillez::formatter::FormatWithAsync;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_result::LintResult;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::MusicbrainzLints;
use crate::utils::formater;

pub struct SuspiciousRemixLint;

pub struct SuspiciousRemixLintRes {
    recording: Recording,
}

impl MbClippyLint for SuspiciousRemixLint {
    type Result = SuspiciousRemixLintRes;

    fn get_name() -> &'static str {
        "suspicious_remix"
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity,
    ) -> Result<Vec<SuspiciousRemixLintRes>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(Vec::new());
        };

        // Check if the title is suspiciously like a remix or vip
        let regex = Regex::new(r"(?mi)(((\(|\s))remix(\)|$))|(((\(|\s))vip(\)|$))").unwrap();

        if !regex.is_match(&recording.title) {
            return Ok(Vec::new());
        }

        let conn = &mut client.mb_database.get_raw_connection().await?;

        // Then check if the links have been set
        if recording.is_remix(conn).await? {
            return Ok(Vec::new());
        }

        Ok(vec![SuspiciousRemixLintRes {
            recording: recording.clone(),
        }])
    }
}

impl LintResult for SuspiciousRemixLintRes {
    fn get_name() -> &'static str {
        SuspiciousRemixLint::get_name()
    }

    async fn get_body(
        &self,

        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" seems to be a remix, but no remix relations have been set
-> Add a `remix of` and `remixer` relationships to the recording",
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

        Ok(out)
    }

    async fn get_hints(
        &self,
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        Ok(Vec::new())
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingRelation
    }
}
