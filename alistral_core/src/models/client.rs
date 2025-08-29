use std::sync::Arc;

use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_db_lite::DBClient;

use crate::datastructures::entity_with_listens::recording::collection::RecordingWithListenStrategy;
use crate::datastructures::entity_with_listens::user::collection::UserWithListensStrategy;

#[derive(bon::Builder)]
pub struct AlistralClient {
    // Subclients
    pub listenbrainz: Arc<ListenbrainzClient>,

    pub musicbrainz_db: Arc<DBClient>,

    // Configuration
    #[builder(default = false)]
    pub offline: bool,

    // Strategies
    #[builder(default)]
    pub user_with_listen_strat: UserWithListensStrategy,

    #[builder(default)]
    pub recording_with_listen_strat: RecordingWithListenStrategy,
}
