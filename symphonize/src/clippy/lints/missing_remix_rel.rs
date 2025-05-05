use musicbrainz_db_lite::FetchAsComplete;
use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};
use tuillez::formatter::FormatWithAsync;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_result::LintResult;
use crate::clippy::lint_severity::LintSeverity;

use crate::SymphonyzeClient;
use crate::clippy::lints::MusicbrainzLints;
use crate::utils::formater;

pub struct MissingRemixRelLint;

impl MbClippyLint for MissingRemixRelLint {
    type Result = MissingRemixRelLintRes;

    fn get_name() -> &'static str {
        "missing_remix_rel"
    }

    async fn prefetch_entities(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<(), crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(());
        };

        recording
            .fetch_as_complete_as_task(client.mb_database.clone())
            .await?;

        Ok(())
    }

    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<MissingRemixRelLintRes>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(Vec::new());
        };

        let conn = &mut client.mb_database.get_raw_connection().await?;
        let recording_rels = recording.get_artist_relations(conn).await?;
        let mut is_remix = false;
        for relation in recording_rels {
            if relation.is_remixer_rel(recording) {
                is_remix = true;
            }
        }

        if !is_remix {
            return Ok(Vec::new());
        }

        let artist_relations = recording.get_recording_relations(conn).await?;
        // Check if a remixer relationship is missing
        for relation in artist_relations {
            if relation.is_remix_of_rel(recording) {
                return Ok(Vec::new());
            }
        }

        let lint = MissingRemixRelLintRes {
            recording: recording.clone(),
        };

        Ok(vec![lint])
    }
}

pub struct MissingRemixRelLintRes {
    recording: Recording,
}

impl LintResult for MissingRemixRelLintRes {
        fn get_name() -> &'static str {
        MissingRemixRelLint::get_name()
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" has a remixer relationship, but no `remix of` relationship.
-> Add the original recording as an recording relationship",
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

