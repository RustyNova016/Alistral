use musicbrainz_db_lite::models::musicbrainz::{main_entities::MainEntity, recording::Recording};

use crate::models::clippy::lint_severity::LintSeverity;
use crate::models::clippy::{MbClippyLint, MbClippyLintLink};
use crate::utils::cli::display::RecordingExt;
use crate::utils::extensions::db_lite_ext::RelationRecordingArtistExt;
use crate::utils::extensions::db_lite_ext::RelationRecordingRecordingExt;

pub struct MissingRemixRelLint {
    recording: Recording,
}

impl MbClippyLint for MissingRemixRelLint {
    fn get_name() -> &'static str {
        "missing_remix_rel"
    }

    async fn check(
        conn: &mut sqlx::SqliteConnection,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Recording(recording) = entity else {
            return Ok(None);
        };

        let recording_rels = recording.get_artist_relations(conn).await?;
        let mut is_remix = false;
        for relation in recording_rels {
            if relation.is_remixer_rel(recording) {
                is_remix = true;
            }
        }

        if !is_remix {
            return Ok(None);
        }

        let artist_relations = recording.get_recording_relations(conn).await?;
        // Check if a remixer relationship is missing
        for relation in artist_relations {
            if relation.is_remix_of_rel(recording) {
                return Ok(None);
            }
        }

        let lint = Self {
            recording: recording.clone(),
        };

        Ok(Some(lint))
    }

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Recording \"{}\" has a remixer relationship, but no `remix of` relationship.
-> Add the original recording as an recording relationship",
            self.recording
                .pretty_format_with_credits(conn, false)
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
