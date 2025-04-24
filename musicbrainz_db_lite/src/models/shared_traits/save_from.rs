use std::sync::Arc;

use crate::DBClient;

pub trait SaveFrom<T>
where
    Self: Sized,
    T: Send,
{
    fn save_from(
        conn: &mut sqlx::SqliteConnection,
        data: T,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send;

    fn save_from_with_pool(
        client: &DBClient,
        data: T,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send {
        async {
            let conn = &mut client.get_raw_connection().await?;
            Self::save_from(conn, data).await
        }
    }

    fn save_from_as_task(
        client: Arc<DBClient>,
        data: T,
    ) -> impl std::future::Future<Output = Result<Self, crate::Error>> + Send
    where
        Self: Send + 'static,
        T: 'static,
    {
        async {
            tokio::spawn(async move {
                let client = client.clone();
                Self::save_from_with_pool(client.as_ref(), data).await
            })
            .await
            .unwrap()
        }
    }
}
