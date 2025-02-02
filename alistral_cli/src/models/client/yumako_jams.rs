use std::sync::Arc;

use alistral_core::AlistralClient;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use alistral_core::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use musicbrainz_db_lite::DBClient;

use crate::models::client::AlistralCliClient;
use crate::utils::env::in_offline_mode;

impl AlistralCliClient {
    pub fn create_yumako_jams_client(alistral_core: Arc<AlistralClient>) -> Arc<YumakoClient> {
        Arc::new(YumakoClient { alistral_core })
    }
}
