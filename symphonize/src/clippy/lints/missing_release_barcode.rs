use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::release::Release;
use tuillez::formatter::FormatWithAsync;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_result::LintResult;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::MusicbrainzLints;
use crate::utils::formater;

pub struct MissingBarcodeLint;

impl MbClippyLint for MissingBarcodeLint {
    type Result = MissingBarcodeLintRes;
    fn get_name() -> &'static str {
        "missing_release_barcode"
    }

    async fn check(
        _client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Vec<MissingBarcodeLintRes>, crate::Error> {
        let MainEntity::Release(release) = entity else {
            return Ok(Vec::new());
        };

        if release.barcode.as_ref().is_some() {
            return Ok(Vec::new());
        }

        let missing_work_lint = MissingBarcodeLintRes {
            release: release.clone(),
        };

        Ok(vec![missing_work_lint])
    }
}

pub struct MissingBarcodeLintRes {
    release: Release,
}

impl LintResult for MissingBarcodeLintRes {
    fn get_name() -> &'static str {
        MissingBarcodeLint::get_name()
    }

    async fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        Ok(format!(
            "Release \"{}\" has no barcode
-> No barcode has been entered for this release, nor has been set has not having one.",
            self.release.format_with_async(&formater(client)).await?
        ))
    }

    async fn get_links(
        &self,
        _client: &SymphonyzeClient,
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
        _client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error> {
        let hints = Vec::new();

        // TODO: #525 missing_release_barcode: add hint to use harmony to find it if possible

        Ok(hints)
    }

    fn get_severity(&self) -> LintSeverity {
        LintSeverity::MissingData
    }
}
