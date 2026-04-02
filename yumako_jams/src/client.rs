use std::sync::Arc;

use alistral_core::AlistralClient;
use musicbrainz_db_lite::GetConnectionError;
use musicbrainz_db_lite::SqlitePoolConnection;

pub struct YumakoClient {
    pub alistral_core: Arc<AlistralClient>,
}

impl YumakoClient {
    /// Retrieve a reference to a `musicbrainz_db_lite` connection
    // pub async fn get_db_lite_conn(&self) -> DBLitePoolResult {
    //     self.alistral_core.musicbrainz_db.get_conn().await
    // }

    /// Retrieve a reference to a raw `musicbrainz_db_lite` connection
    pub async fn get_db_lite_raw_conn(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        self.alistral_core.musicbrainz_db.get_raw_connection().await
    }
}
