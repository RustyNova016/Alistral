use std::sync::Arc;

use crate::DBClient;
use crate::Release;
use crate::models::shared_traits::db_relation::EntityActiveURLDBRel;
use crate::utils::regex::HARMONY_SEED_URL_REGEX;

impl Release {
    /// Return true if the release is able to be seeded in harmony. This means the release has a link from a supported provider
    ///
    /// This fetches data if neccesary using tasks.
    pub async fn is_harmony_compatible(
        &self,
        client: &Arc<DBClient>,
    ) -> Result<bool, crate::Error> {
        let urls = self
            .get_related_entity_or_fetch_as_task::<EntityActiveURLDBRel>(client)
            .await?;

        Ok(urls
            .into_iter()
            .any(|url| HARMONY_SEED_URL_REGEX.is_match(&url.ressource)))
    }
}
