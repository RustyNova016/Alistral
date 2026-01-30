use bon::bon;
use snafu::ResultExt;

use crate::DBClient;
use crate::models::listenbrainz::listen::Listen;
use crate::models::listenbrainz::listen::selects::error::ConnectionSnafu;
use crate::models::listenbrainz::listen::selects::error::ListenFetchGetError;
use crate::models::listenbrainz::listen::selects::error::ListenFetchingSnafu;
use crate::models::listenbrainz::listen::selects::error::ListenSelectSnafu;

#[bon]
impl Listen {
    /// All in one function to fetch and get listens.
    #[builder]
    pub async fn get_or_fetch_listens(
        client: &DBClient,
        users: &[&str],
        #[builder(default)] mapped: bool,
        #[builder(default)] unmapped: bool,
        #[builder(default = true)] incremental: bool,
    ) -> Result<Vec<Listen>, ListenFetchGetError> {
        for user in users {
            Listen::fetch_and_insert(client, user, incremental)
                .await
                .context(ListenFetchingSnafu)?;
        }

        let query = Listen::listen_query_string()
            .unmapped(unmapped)
            .mapped(mapped)
            .users(users)
            .call();

        sqlx::query_as(&query)
            .fetch_all(&mut *client.get_conn().await.context(ConnectionSnafu)?)
            .await
            .context(ListenSelectSnafu)
    }
}
