use std::sync::Arc;

use musicbrainz_db_lite::listenbrainz_rs::ListenBrainzAPIEnpoints;
use musicbrainz_db_lite::listenbrainz_rs::ListenBrainzClient;

use crate::models::client::AlistralCliClient;
use crate::models::config::Config;

impl AlistralCliClient {
    pub(super) fn create_lb_client(config: &Config) -> Arc<ListenBrainzClient> {
        Arc::new(
            ListenBrainzClient::builder()
                .endpoints(
                    ListenBrainzAPIEnpoints::builder()
                        .lb_domain(config.listenbrainz_domain.clone())
                        .build(),
                )
                .build(),
        )
    }
}
