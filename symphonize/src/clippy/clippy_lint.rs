use musicbrainz_db_lite::models::musicbrainz::main_entities::MainEntity;

use crate::client::SymphonyzeClient;
use crate::clippy::lint_result::LintResult;

pub trait MbClippyLint {
    type Result: LintResult;

    /// Fetch all the required entities that the lint requires if they aren't alreasy in the database.
    fn prefetch_entities(
        _client: &SymphonyzeClient,
        _entity: &MainEntity,
    ) -> impl std::future::Future<Output = Result<(), crate::Error>> + Send {
        async { Ok(()) }
    }

    /// Check if the entity triggers the lint, and output lints entities for each problem encountered
    fn check(
        client: &SymphonyzeClient,
        entity: &MainEntity,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Result>, crate::Error>> + Send;

    fn get_name() -> &'static str;
}
