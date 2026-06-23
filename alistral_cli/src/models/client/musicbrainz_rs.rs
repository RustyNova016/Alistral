use std::sync::Arc;

use musicbrainz_db_lite::MusicBrainzClient;

use crate::models::client::AlistralCliClient;
use crate::models::config::Config;

impl AlistralCliClient {
    pub fn create_mb_client(config: &Config) -> Arc<MusicBrainzClient> {
        let client = MusicBrainzClient::builder();
        let server = config.get_musicbrainz_server();

        let client = client.musicbrainz_use_https(server.use_https);
        let client = client.musicbrainz_authority(server.authority.to_owned());

        let mut client = client.build();

        if let Some(ratelimit) = &server.ratelimit {
            client.api_client.rate_limit = Some(Arc::new(ratelimit.as_rate_limiter()));
        }

        Arc::new(client)
    }
}
