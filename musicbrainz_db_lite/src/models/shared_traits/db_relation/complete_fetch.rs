use std::sync::Arc;

use crate::DBClient;
use crate::DBRelation;
use crate::FetchAsComplete;

/// Extention trait for all the fetchable entities that needs to be complete to get all the data
pub trait DBRelationFetch<T, U>
where
    Self: DBRelation<T> + FetchAsComplete<U> + Sync + Clone,
    U: Send,
{
    /// Make sure the entity is complete, and get the related entity
    fn get_related_entity_or_fetch_with_conn(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &DBClient,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ReturnedType>, crate::Error>> + Send
    {
        async {
            self.fetch_as_complete_with_conn(conn, client)
                .await?
                .get_related_entity(conn)
                .await
        }
    }

    /// Make sure the entity is complete, and get the related entity
    fn get_related_entity_or_fetch_with_pool(
        &self,
        client: &DBClient,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ReturnedType>, crate::Error>> + Send
    {
        async {
            self.fetch_as_complete_with_pool(client)
                .await?
                .get_related_entity(&mut *client.get_raw_connection().await?)
                .await
        }
    }

    /// Make sure the entity is complete, and get the related entity
    fn get_related_entity_or_fetch_as_task(
        &self,
        client: &Arc<crate::DBClient>,
    ) -> impl std::future::Future<Output = Result<Vec<Self::ReturnedType>, crate::Error>> + Send
    where
        Self: 'static,
        U: 'static,
    {
        async move {
            self.fetch_as_complete_as_task(client.clone())
                .await?
                .get_related_entity(&mut *client.get_raw_connection().await?)
                .await
        }
    }
}

impl<T, U, R> DBRelationFetch<R, U> for T
where
    T: Send + Sync + DBRelation<R> + FetchAsComplete<U> + Clone,
    U: Send,
{
}
