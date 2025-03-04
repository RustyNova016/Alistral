use std::sync::Arc;

use alistral_core::AlistralClient;
use musicbrainz_db_lite::database::pool::DBLitePoolResult;
use musicbrainz_db_lite::database::raw_conn_pool::RawPoolResult;

pub struct YumakoClient {
    pub alistral_core: Arc<AlistralClient>,
}

impl YumakoClient {
    /// Retrieve a reference to a `musicbrainz_db_lite` connection
    pub async fn get_db_lite_conn(&self) -> DBLitePoolResult {
        self.alistral_core.musicbrainz_db.get_connection().await
    }

    /// Retrieve a reference to a raw `musicbrainz_db_lite` connection
    pub async fn get_db_lite_raw_conn(&self) -> RawPoolResult {
        self.alistral_core.musicbrainz_db.get_raw_connection().await
    }
}
