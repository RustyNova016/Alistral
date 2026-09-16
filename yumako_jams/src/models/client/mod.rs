use std::sync::Arc;

use alistral_core::AlistralClient;
use musicbrainz_db_lite::GetConnectionError;
use musicbrainz_db_lite::SqlitePoolConnection;

pub struct YumakoClient {
    pub alistral_core: Arc<AlistralClient>,
    //pub interzic: Arc<InterzicClient>
}

impl YumakoClient {
    // Current version of yumako_jams
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    /// Retrieve a reference to a raw `musicbrainz_db_lite` connection
    pub async fn get_db_lite_raw_conn(&self) -> Result<SqlitePoolConnection, GetConnectionError> {
        self.alistral_core.musicbrainz_db.get_raw_connection().await
    }
}
