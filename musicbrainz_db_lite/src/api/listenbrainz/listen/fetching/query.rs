use core::ops::DerefMut;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use futures::join;
use futures::FutureExt;
use listenbrainz::raw::response::UserListensListen;
use sqlx::Acquire;
use thiserror::Error;
use tracing::info;

use crate::api::listenbrainz::listen::fetching::api::fetch_user_listens;
use crate::models::listenbrainz::listen::Listen;
use crate::DBClient;

pub struct ListenFetchAPIQuery {
    user: String,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
    count: u32,
}

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

        Ok(Self {
            user,
            from,
            to,
            count,
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
            latest_listen.map(|l| l.listened_at_as_datetime()),
            Some(Utc::now()),
            1000,
        )?)
    }

    pub async fn request_and_save(&mut self, client: &DBClient) -> Result<(), crate::Error> {
        let result = self.request(client).await?;

        let mut conn = client.get_raw_connection().await?;
        let mut trans = conn.deref_mut().begin().await?;

        info!("Saving listens...");
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
            Self::try_new(
                self.user.clone(),
                self.from,
                DateTime::from_timestamp(middle + 1, 0),
                self.count,
            )
            .unwrap(),
            Self::try_new(
                self.user.clone(),
                DateTime::from_timestamp(middle, 0),
                self.to,
                self.count,
            )
            .unwrap(),
        ))
    }

    async fn split_and_request(
        &mut self,
        client: &DBClient,
    ) -> Result<Vec<UserListensListen>, reqwest::Error> {
        let (mut a, mut b) = self.split().unwrap();

        let (a_iter, b_iter) = join!(a.request(client), b.request(client));

        let mut a_vec = a_iter?;
        let b_vec = b_iter?;

        a_vec.extend(b_vec);

        Ok(a_vec)
    }

    async fn request(
        &mut self,
        client: &DBClient,
    ) -> Result<Vec<UserListensListen>, reqwest::Error> {
        // If the work is too big, split it
        if self
            .fetch_interval_duration()
            .is_some_and(|d| d > Duration::days(30))
        {
            return self.split_and_request(client).boxed_local().await;
        }

        // Fetch from the api
        info!(
            "Fetching listens from {} to {}",
            self.from.unwrap_or(DateTime::from_timestamp(0, 0).unwrap()),
            self.to.unwrap_or(Utc::now())
        );
        let response = fetch_user_listens(
            client,
            &self.user,
            self.from.map(|t| t.timestamp()),
            self.to.map(|t| t.timestamp()),
            Some(self.count),
        )
        .await?;

        // Did we fetch everything?
        if self.count as u64 > response.payload.count {
            // Then everything is fine to yield!
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
