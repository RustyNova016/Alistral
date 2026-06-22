use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;
use tracing::warn;

use crate::models::config::Config;
use crate::models::config::ratelimit::RateLimitConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicbrainzServer {
    pub use_https: bool,
    pub authority: String,

    pub ratelimit: Option<RateLimitConfig>,
}

impl Default for MusicbrainzServer {
    fn default() -> Self {
        Self {
            use_https: true,
            authority: "musicbrainz.org".to_string(),
            ratelimit: None,
        }
    }
}

impl Config {
    pub fn get_musicbrainz_server(&self) -> Cow<'_, MusicbrainzServer> {
        if let Some(server) = &self.musicbrainz_server {
            return Cow::Borrowed(server);
        }

        if let Some(server) = &self.musicbrainz_url {
            if server == "http://musicbrainz.org/ws/2"
                || server == "musicbrainz.org/ws/2"
                || server == "musicbrainz.org"
            {
                return Cow::Owned(MusicbrainzServer::default());
            }

            warn!("Config `musicbrainz_url` is deprecated. Please migrate to `musicbrainz_server`");
            return Cow::Owned(MusicbrainzServer {
                authority: server.to_owned(),
                ratelimit: None,
                use_https: true,
            });
        }

        Cow::Owned(MusicbrainzServer::default())
    }
}
