use std::sync::Arc;

use alistral_core::AlistralClient;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use musicbrainz_db_lite::DBClient;

use crate::models::client::AlistralCliClient;
use crate::utils::env::in_offline_mode;

impl AlistralCliClient {
    pub(super) fn create_core_client(musicbrainz_db: Arc<DBClient>) -> Arc<AlistralClient> {
        AlistralClient::builder()
            .musicbrainz_db(musicbrainz_db)
            .offline(in_offline_mode())
            .recording_with_listen_strat(RecordingWithListenStrategy::default())
            .user_with_listen_strat(UserWithListensStrategy::default())
            .build()
            .into()
    }

    pub async fn statistics_of_user(&self, username: String) -> ListenStatisticsData {
        ListenStatisticsData::new_from_user_listens(self.core.clone(), username)
            .await
            .unwrap()
    }
}
