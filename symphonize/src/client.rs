use std::sync::Arc;

use musicbrainz_db_lite::DBClient;

pub struct SymphonyzeClient {
    pub mb_database: Arc<DBClient>,
}
