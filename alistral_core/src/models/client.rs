use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::database::client::DBClient;

pub struct AlistralClient {
    // Subclients
    pub listenbrainz: Arc<ListenbrainzClient>,

    pub musicbrainz_db: Arc<DBClient>,

    // Configuration
    pub offline: bool,
}
