use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;
use musicbrainz_db_lite::models::musicbrainz::work::Work;

use crate::models::clippy::MbClippyLint;

pub struct MissingWorkLanguageLint {
    work: Work
}

impl MbClippyLint for MissingWorkLanguageLint {
    async fn check(
        conn: &mut sqlx::SqliteConnection,
        entity: &musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity,
    ) -> Result<Option<Self>, crate::Error> {
        let MainEntity::Work(work) = entity else {
            return Ok(None);
        };
    }

    fn get_name() -> &'static str {
        todo!()
    }

    async fn get_body(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<impl std::fmt::Display, crate::Error> {
        todo!()
    }

    async fn get_links(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintLink>, crate::Error> {
        todo!()
    }

    async fn get_hints(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Vec<crate::models::clippy::MbClippyLintHint>, crate::Error> {
        todo!()
    }
}