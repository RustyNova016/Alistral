use core::time::Duration;

use chrono::DateTime;
use chrono::Utc;
use listenbrainz::raw::response::RateLimit;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tokio::sync::Semaphore;

pub struct RateLimiter {
    remaining: Semaphore,
    reset_at: RwLock<DateTime<Utc>>,

    /// Lock the semaphore from getting tickets
    aquire_lock: RwLock<bool>
}

impl RateLimiter {
    pub async fn wait(&self) {
        let lock = self.aquire_lock.read().await;
        let ticket = self.remaining.acquire().await.unwrap();
        ticket.forget();
    }

    pub async fn set_return(&self, rate_limit: RateLimit) {
        let lock = self.aquire_lock.write().await;    
        let reset_at = Utc::now() + Duration::from_secs(num_secs);
        let this_reset = self.reset_at.write().await;
        if this_reset < reset_at {
            *this_reset
        }
    }
}