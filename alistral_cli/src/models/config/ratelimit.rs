use core::num::NonZeroU32;

use musicbrainz_db_lite::api_bindium::governor::Quota;
use musicbrainz_db_lite::api_bindium::governor::RateLimiter;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    tokens: u32,
    each: u32,
    max_burst: Option<u32>,
}

impl RateLimitConfig {
    pub fn musicbrainz_default() -> Self {
        Self {
            tokens: 10,
            each: 10,
            max_burst: Some(5),
        }
    }

    pub fn as_rate_limiter(
        &self,
    ) -> RateLimiter<
        musicbrainz_db_lite::api_bindium::governor::state::NotKeyed,
        musicbrainz_db_lite::api_bindium::governor::state::InMemoryState,
        musicbrainz_db_lite::api_bindium::governor::clock::QuantaClock,
        musicbrainz_db_lite::api_bindium::governor::middleware::NoOpMiddleware<
            musicbrainz_db_lite::api_bindium::governor::clock::QuantaInstant,
        >,
    > {
        // The interface is rather bad so we need to calculate the token per hours allowed
        let token_hour = (self.tokens * 3600) / self.each;

        let quota = Quota::per_hour(NonZeroU32::new(token_hour).unwrap())
            .allow_burst(NonZeroU32::new(self.max_burst.unwrap_or(1)).unwrap());
        RateLimiter::direct(quota)
    }
}
