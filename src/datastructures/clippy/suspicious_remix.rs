use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use regex::Regex;

use crate::ALISTRAL_CLIENT;
use crate::models::clippy::MbClippyLint;
use crate::models::clippy::MbClippyLintLink;
use crate::models::clippy::lint_severity::LintSeverity;

pub struct SuspiciousRemixLint {
    recording: Recording,
}

impl MbClippyLint for SuspiciousRemixLint {
    fn get_name() -> &'static str {
        "suspicious_remix"
    }

    async fn check(
        conn: &mut sqlx::SqliteConnection,
        entity: &musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        // Check if the title is suspiciously like a remix or vip
        let regex = Regex::new(r"(?mi)(((\(|\s))remix(\)|$))|(((\(|\s))vip(\)|$))").unwrap();

        if !regex.is_match(&recording.title) {
            return Ok(None);
        }

        // Then check if the links have been set
        if recording.is_remix(conn).await? {
            return Ok(None);
        }

        Ok(Some(Self {
            recording: recording.clone(),
        }))
    }

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" seems to be a remix, but no remix relations have been set
-> Add a `remix of` and `remixer` relationships to the recording",
            self.recording
                .pretty_format_with_credits(conn, &ALISTRAL_CLIENT.musicbrainz_db, false)
                .await?
        ))
    }

    async fn get_links(
        &self,
        _conn: &mut sqlx::SqliteConnection,
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
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintHint>, crate::Error> {
        Ok(Vec::new())
    }

    fn get_severity(&self) -> crate::models::clippy::lint_severity::LintSeverity {
        LintSeverity::MissingRelation
    }
}
