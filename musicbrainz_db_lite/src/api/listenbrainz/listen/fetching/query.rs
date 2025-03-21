use core::num::NonZeroU32;
use core::ops::DerefMut;
use std::sync::Arc;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use futures::join;
use futures::FutureExt;
use governor::Quota;
use governor::RateLimiter;
use listenbrainz::raw::response::UserListensListen;
use sqlx::Acquire;
use thiserror::Error;
use tracing::debug;
use tracing::instrument;
use tracing::Span;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;
use tuillez::tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::api::listenbrainz::listen::fetching::api::fetch_user_listens;
use crate::models::listenbrainz::listen::Listen;
use crate::DBClient;

pub struct ListenFetchAPIQuery {
    user: String,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    count: u32,

    rate_limiter: RateLimiterType,
}

type RateLimiterType = Arc<
    RateLimiter<
        governor::state::NotKeyed,
        governor::state::InMemoryState,
        governor::clock::QuantaClock,
        governor::middleware::NoOpMiddleware<governor::clock::QuantaInstant>,
    >,
>;

impl ListenFetchAPIQuery {
    pub fn try_new(
        user: String,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        count: u32,
    ) -> Result<Self, ListenFetchQueryError> {
        if count == 0 || count > 1000 {
            return Err(ListenFetchQueryError::InvalidCount(count));
        }

        if from.is_none() && to.is_none() {
            return Err(ListenFetchQueryError::NoDates);
        }

        let quota = Quota::per_second(NonZeroU32::new(2).unwrap())
            .allow_burst(NonZeroU32::new(15).unwrap());
        let rate_limiter = Arc::new(RateLimiter::direct(quota));

        Ok(Self {
            user,
            from,
            to,
            count,
            rate_limiter,
        })
    }

    pub async fn incremental_fetch_user(
        client: &DBClient,
        user: String,
    ) -> Result<Self, crate::Error> {
        let mut conn = client.get_raw_connection().await?;
        let latest_listen = Listen::get_latest_listen_of_user(&mut conn, &user).await?;

        Ok(Self::try_new(
            user,
            // Offset by 3 days to catch some remappings
            latest_listen.map(|l| l.listened_at_as_datetime() - Duration::days(3)), // TODO: Add to client config
            Some(Utc::now()),
            1000,
        )?)
    }

    #[instrument(skip(self, client), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn request_and_save(&mut self, client: &DBClient) -> Result<(), crate::Error> {
        pg_counted!(1, "Fetching Listens");
        let result = self.request(client).await?;

        let oldest_ts = result.iter().map(|l| l.listened_at).min().unwrap_or(-1);
        let latest_ts = result.iter().map(|l| l.listened_at).max().unwrap_or(-1);

        let mut conn = client.get_raw_connection().await?;
        let mut trans = conn.deref_mut().begin().await?;

        pg_spinner!("Saving listens...");
        Listen::delete_listen_period(&mut trans, oldest_ts, latest_ts, &self.user).await?;

        for lis in result {
            Listen::insert_api_listen(&mut trans, &lis).await?;
        }

        Ok(())
    }

    pub fn get_from_ts(&self) -> Option<i64> {
        self.from.map(|d| d.timestamp())
    }

    pub fn get_to_ts(&self) -> Option<i64> {
        self.from.map(|d| d.timestamp())
    }

    /// Split the work into two requests
    fn split(&self) -> Option<(Self, Self)> {
        let from = self.get_from_ts()?;
        let dur = self.fetch_interval_duration()?.num_seconds();

        let middle = (dur / 2) + from;

        Some((
            Self {
                user: self.user.clone(),
                from: self.from,
                to: DateTime::from_timestamp(middle + 1, 0),
                count: self.count,
                rate_limiter: self.rate_limiter.clone(),
            },
            Self {
                user: self.user.clone(),
                from: DateTime::from_timestamp(middle, 0),
                to: self.to,
                count: self.count,
                rate_limiter: self.rate_limiter.clone(),
            },
        ))
    }

    async fn split_and_request(
        &mut self,
        client: &DBClient,
    ) -> Result<Vec<UserListensListen>, crate::Error> {
        let (mut a, mut b) = self.split().unwrap();

        Span::current().pb_inc_length(1);

        let (a_iter, b_iter) = join!(a.request(client), b.request(client));

        let mut a_vec = a_iter?;
        let b_vec = b_iter?;

        a_vec.extend(b_vec);

        Ok(a_vec)
    }

    async fn request(&mut self, client: &DBClient) -> Result<Vec<UserListensListen>, crate::Error> {
        // If the work is too big, split it
        if self
            .fetch_interval_duration()
            .is_some_and(|d| d > Duration::days(15))
        {
            return self.split_and_request(client).boxed_local().await;
        }

        // Fetch from the api
        self.wait_for_rate_limit().await;
        debug!(
            "Fetching listens from {} to {}",
            self.from.unwrap_or(DateTime::from_timestamp(0, 0).unwrap()),
            self.to.unwrap_or(Utc::now())
        );
        let response = fetch_user_listens(
            client,
            &self.user,
            self.from.map(|t| t.timestamp() - 1),
            self.to.map(|t| t.timestamp() + 1),
            Some(self.count),
        )
        .await?;

        // Did we fetch everything?
        if self.count as u64 > response.payload.count {
            // Then everything is fine to yield!
            pg_inc!();
            return Ok(response.payload.listens);
        }

        // We cannot be sure to have fetched everything, so we have to split...
        // But we need to set the bounds first
        if self.from.is_none() {
            self.from = DateTime::from_timestamp(response.payload.oldest_listen_ts, 0);
        }

        if self.to.is_none() {
            self.to = DateTime::from_timestamp(response.payload.latest_listen_ts, 0);
        }

        // Bounds set. We split and yield that
        self.split_and_request(client).boxed_local().await
    }

    async fn wait_for_rate_limit(&self) {
        self.rate_limiter.until_ready().await;
    }

    fn fetch_interval_duration(&self) -> Option<Duration> {
        Some(self.to? - self.from?)
    }
}

#[derive(Error, Debug)]
pub enum ListenFetchQueryError {
    #[error("The fetch count must be between 1 and 1000 (inclusive), but tried to fetch {0}")]
    InvalidCount(u32),

    #[error("`from` and `to` cannot be both `None`")]
    NoDates,
}
