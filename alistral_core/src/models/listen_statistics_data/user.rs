use crate::datastructures::entity_with_listens::user::collection::UserWithListensCollection;
use crate::models::listen_statistics_data::ListenStatisticsData;

impl ListenStatisticsData {
    /// Generate the user statistics based on the stored listens
    pub async fn user_stats(&self) -> Result<&UserWithListensCollection, crate::Error> {
        self.users.get_or_try_init(self.init_user()).await
    }

    /// Create the user statistics
    async fn init_user(&self) -> Result<UserWithListensCollection, crate::Error> {
        UserWithListensCollection::from_listencollection(
            &self.client,
            self.listens.clone(),
            &self.client.user_with_listen_strat,
        )
        .await
    }
}
