use core::future::Future;
use std::sync::Arc;

use sequelles::has_rowid::HasRowID;

use crate::DBClient;
use crate::models::shared_traits::HasMBID;
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_mbid::FetchMBID;
use crate::models::shared_traits::save_from::SaveFrom;

pub trait FetchAndSave<U>
where
    Self: Sized + FetchMBID<U> + SaveFrom<U> + HasRowID + CompletenessFlag,
    U: Send,
{
    /// Fetch save an entity using a premade connection
    fn fetch_and_save_with_conn(
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: std::marker::Send,
    {
        async {
            let data = Self::fetch_from_mbid(client, mbid).await;

            match data {
                Ok(data) => {
                    let mut data = Self::save_from(conn, data).await?;
                    data.set_full_update(conn).await?;

                    Self::set_redirection(conn, mbid, data.rowid()).await?;

                    Ok(Some(data))
                }
                Err(err) => {
                    if err
                        .as_musicbrainz_error()
                        .is_some_and(|err| err.is_not_found())
                    {
                        // TODO: Set deleted
                        Ok(None)
                    } else {
                        Err(err.into())
                    }
                }
            }
        }
    }

    /// Fetch save an entity using the connection pool of the client
    fn fetch_and_save_with_pool(
        client: &crate::DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: Send,
    {
        async {
            let conn = &mut client.get_raw_connection().await?;
            Self::fetch_and_save_with_conn(conn, client, mbid).await
        }
    }

    /// Fetch save an entity using the connection pool of the client, and create a tokio task for it.
    /// This allows to send save requests that are sure to complete (and unlock the database)
    /// even if the returned future isn't polled anymore
    fn fetch_and_save_as_task(
        client: Arc<DBClient>,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send
    where
        Self: Send + 'static,
        U: 'static,
    {
        async {
            let data = Self::fetch_from_mbid_as_task(client.clone(), mbid).await;

            match data {
                Ok(data) => {
                    let mbid = mbid.to_string();

                    tokio::spawn(async move {
                        let conn = &mut client.get_raw_connection().await?;
                        let mut data = Self::save_from(conn, data).await?;
                        data.set_full_update(conn).await?;

                        Self::set_redirection(conn, &mbid, data.rowid()).await?;

                        Ok(Some(data))
                    })
                    .await
                    .unwrap()
                }
                Err(err) => {
                    if err
                        .as_musicbrainz_error()
                        .is_some_and(|err| err.is_not_found())
                    {
                        // TODO: Set deleted
                        Ok(None)
                    } else {
                        Err(err.into())
                    }
                }
            }
        }
    }

    /// Set the MBID redirection from a given MBID to the current entity's rowid
    fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    // Refresh the data in the database by refetching the entity, using an existing connection
    fn refetch_with_conn(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: HasMBID + Send + Sync,
    {
        async {
            Self::fetch_and_save_with_conn(conn, client, self.get_mbid())
                .await?
                .ok_or(crate::Error::NotFoundInUpstream(
                    self.get_mbid().to_string(),
                ))
        }
    }

    // Refresh the data in the database by refetching the entity, using the client's connection pool
    fn refetch_with_pool(
        &self,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: HasMBID + Send + Sync,
    {
        async {
            Self::fetch_and_save_with_pool(client, self.get_mbid())
                .await?
                .ok_or(crate::Error::NotFoundInUpstream(
                    self.get_mbid().to_string(),
                ))
        }
    }

    // Refresh the data in the database by refetching the entity, using a tokio task for **saving** the entity to the database. Fetching remains a regular future
    fn refetch_as_task(
        &self,
        client: Arc<crate::DBClient>,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: HasMBID + Send + Sync + 'static,
        U: 'static,
    {
        async {
            Self::fetch_and_save_as_task(client, self.get_mbid())
                .await?
                .ok_or(crate::Error::NotFoundInUpstream(
                    self.get_mbid().to_string(),
                ))
        }
    }
}
