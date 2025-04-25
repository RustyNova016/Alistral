use std::sync::Arc;

use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::find_by_mbid::FindByMBID;

pub trait GetOrFetch<U>
where
    Self: FetchAndSave<U> + FindByMBID + Send,
    U: Send,
{
    /// Get or fetch an entity using an existing connection
    fn get_or_fetch_with_conn(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send {
        async {
            let data = Self::find_by_mbid(conn, mbid).await?;

            match data {
                Some(val) => Ok(Some(val)),
                None => Self::fetch_and_save_with_conn(conn, client, mbid).await,
            }
        }
    }

    /// Get or fetch an entity using the pool of the client
    fn get_or_fetch_with_pool(
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send {
        async {
            let conn = &mut *client.get_raw_connection().await?;

            Self::get_or_fetch_with_conn(conn, client, mbid).await
        }
    }

    /// Get or fetch an entity using a tokio task for **saving** the entity to the database. Fetching remains a regular future
    fn get_or_fetch_as_task(
        client: Arc<crate::DBClient>,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: 'static,
        U: 'static,
    {
        async {
            let data = Self::find_by_mbid(&mut *client.get_raw_connection().await?, mbid).await?;

            match data {
                Some(val) => Ok(Some(val)),
                None => Self::fetch_and_save_as_task(client, mbid).await,
            }
        }
    }
}

impl<T, U> GetOrFetch<U> for T
where
    Self: FetchAndSave<U> + FindByMBID + Send,
    U: Send,
{
}
