use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::release::Release;

use crate::models::clippy::{MbClippyLint, MbClippyLintLink};

pub struct MissingBarcodeLint {
    release: Release,
}

impl MbClippyLint for MissingBarcodeLint {
    fn get_name() -> &'static str {
        "missing_release_barcode"
    }

    async fn check(
        conn: &mut sqlx::SqliteConnection,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Release(release) = entity else {
            return Ok(None);
        };

        let mut release = release.clone();

        release.refetch_and_load(conn).await?;

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
            self.release.format_with_credits(conn).await?
        ))
    }

    async fn get_links(
        &self,
        conn: &mut sqlx::SqliteConnection,
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
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintHint>, crate::Error> {
        let mut hints = Vec::new();

        // TODO: Harmony hint

        Ok(hints)
    }
}
