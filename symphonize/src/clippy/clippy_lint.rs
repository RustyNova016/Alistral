use core::fmt::Display;

use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;

use crate::client::SymphonyzeClient;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;

pub trait MbClippyLint: Sized {
    async fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> Result<Option<Self>, crate::Error>;

    fn get_name() -> &'static str;

    async fn get_body(&self, client: &SymphonyzeClient) -> Result<impl Display, crate::Error>;

    async fn get_links(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintLink>, crate::Error>;

    async fn get_hints(
        &self,
        client: &SymphonyzeClient,
    ) -> Result<Vec<MbClippyLintHint>, crate::Error>;

    fn get_severity(&self) -> LintSeverity;
}
