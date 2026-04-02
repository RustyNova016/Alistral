use std::sync::Arc;

use alistral_core::AlistralClient;
use yumako_jams::client::YumakoClient;

use crate::models::client::AlistralCliClient;

impl AlistralCliClient {
    pub fn create_yumako_jams_client(alistral_core: Arc<AlistralClient>) -> Arc<YumakoClient> {
        Arc::new(YumakoClient { alistral_core })
    }
}
