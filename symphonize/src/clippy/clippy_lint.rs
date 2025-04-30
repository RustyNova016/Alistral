use core::fmt::Display;

use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;

use crate::client::SymphonyzeClient;
use crate::clippy::lint_hint::MbClippyLintHint;
use crate::clippy::lint_link::MbClippyLintLink;
use crate::clippy::lint_severity::LintSeverity;
use crate::clippy::lints::MusicbrainzLints;

pub trait MbClippyLint
where
    Self: Sized + Into<MusicbrainzLints>,
{
    fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;

    fn get_name() -> &'static str;

    fn get_name_self(&self) -> &'static str {
        Self::get_name()
    }

    fn get_body(
        &self,
        client: &SymphonyzeClient,
    ) -> impl std::future::Future<Output = Result<impl Display, crate::Error>> + Send;

    fn get_links(
        &self,
        client: &SymphonyzeClient,
    ) -> impl std::future::Future<Output = Result<Vec<MbClippyLintLink>, crate::Error>> + Send;

    fn get_hints(
        &self,
        client: &SymphonyzeClient,
    ) -> impl std::future::Future<Output = Result<Vec<MbClippyLintHint>, crate::Error>> + Send;

    fn get_severity(&self) -> LintSeverity;
}
