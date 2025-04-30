use core::future::Future;
use std::sync::Arc;

use crate::models::shared_traits::HasMBID;
use crate::models::shared_traits::get_or_fetch::GetOrFetch;

pub trait CompletenessFlag {
    /// Reset the "full update" date in the database. This should only be called after a full update of the entity.
    fn set_full_update(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    fn is_complete(&self) -> bool;
}

pub trait FetchAsComplete<U>
where
    Self: GetOrFetch<U> + HasMBID,
    U: Send,
{
    /// Get or fetch an entity using a premade connection, and make sure it is complete
    fn fetch_mbid_as_complete_with_conn(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: Sync,
    {
        async {
            let Some(data) = Self::get_or_fetch_with_conn(conn, client, mbid).await? else {
                return Ok(None);
            };

            // Return the data if complete...
            if data.is_complete() {
                Ok(Some(data))
            } else {
                // ... or refetch
                Ok(Some(data.refetch_with_conn(conn, client).await?))
            }
        }
    }

    fn fetch_as_complete_with_conn(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: Sync + Clone,
    {
        async {
            if self.is_complete() {
                Ok(self.clone())
            } else {
                Self::fetch_mbid_as_complete_with_conn(conn, client, self.get_mbid())
                    .await?
                    .ok_or(crate::Error::NotFoundInUpstream(
                        self.get_mbid().to_string(),
                    ))
            }
        }
    }

    /// Get or fetch an entity using the connection pool of the client, and make sure it is complete
    fn fetch_mbid_as_complete_with_pool(
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: Sync,
    {
        async {
            let Some(data) = Self::get_or_fetch_with_pool(client, mbid).await? else {
                return Ok(None);
            };

            // Return the data if complete...
            if data.is_complete() {
                Ok(Some(data))
            } else {
                // ... or refetch
                Ok(Some(data.refetch_with_pool(client).await?))
            }
        }
    }

    fn fetch_as_complete_with_pool(
        &self,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: Sync + Clone,
    {
        async {
            if self.is_complete() {
                Ok(self.clone())
            } else {
                Self::fetch_mbid_as_complete_with_pool(client, self.get_mbid())
                    .await?
                    .ok_or(crate::Error::NotFoundInUpstream(
                        self.get_mbid().to_string(),
                    ))
            }
        }
    }

    /// Get or fetch an entity, while making sure it is complete, using a tokio task for **saving** the entity to the database. Fetching remains a regular future
    fn fetch_mbid_as_complete_as_task(
        client: Arc<crate::DBClient>,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: Sync + 'static,
        U: 'static,
    {
        async {
            let Some(data) = Self::get_or_fetch_as_task(client.clone(), mbid).await? else {
                return Ok(None);
            };

            // Return the data if complete...
            if data.is_complete() {
                Ok(Some(data))
            } else {
                // ... or refetch
                Ok(Some(data.refetch_as_task(client).await?))
            }
        }
    }

    fn fetch_as_complete_as_task(
        &self,
        client: Arc<crate::DBClient>,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: Sync + 'static + Clone,
        U: 'static,
    {
        async {
            if self.is_complete() {
                Ok(self.clone())
            } else {
                Self::fetch_mbid_as_complete_as_task(client, self.get_mbid())
                    .await?
                    .ok_or(crate::Error::NotFoundInUpstream(
                        self.get_mbid().to_string(),
                    ))
            }
        }
    }
}

impl<T, U> FetchAsComplete<U> for T
where
    Self: GetOrFetch<U> + HasMBID,
    U: Send,
{
}
