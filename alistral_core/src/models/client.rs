use std::sync::Arc;

use interzic::InterzicClient;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::client::MusicBrainzClient;
use musicbrainz_db_lite::database::client::DBClient;

pub struct AlistralClient {
    pub musicbrainz_rs: Arc<MusicBrainzClient>,
    pub listenbrainz: Arc<ListenbrainzClient>,

    pub musicbrainz_db: Arc<DBClient>,
    pub interzic: Arc<InterzicClient>,
}
