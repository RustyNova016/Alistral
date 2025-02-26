use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

pub struct MissingBarcodeLint {
    release: Release,
}

impl MbClippyLint for MissingBarcodeLint {
    fn get_name() -> &'static str {
        "missing_release_barcode"
    }

    async fn check(
        _conn: &mut sqlx::SqliteConnection,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Release(release) = entity else {
            return Ok(None);
        };

        if release.barcode.as_ref().is_some() {
            return Ok(None);
        }

        let missing_work_lint = Self {
            release: release.clone(),
        };

        Ok(Some(missing_work_lint))
    }

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Release \"{}\" has no barcode
-> No barcode has been entered for this release, nor has been set has not having one.",
            self.release.pretty_format_with_credits(conn, false).await?
        ))
    }

    async fn get_links(
        &self,
        _conn: &mut sqlx::SqliteConnection,
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

        Ok(out)
    }

    async fn get_hints(
        &self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let hints = Vec::new();

        // TODO: #525 missing_release_barcode: add hint to use harmony to find it if possible

        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingData
    }
}
