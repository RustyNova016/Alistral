use std::sync::Arc;

use deadpool::managed::Object;
use deadpool::managed::PoolError;
use listenbrainz::raw::Client as ListenbrainzClient;
use musicbrainz_rs_nova::client::MusicBrainzClient;

use crate::database::client::builder::ClientBuilder;
use crate::database::pool::DBLitePool;
use crate::database::pool::DBLitePoolExt;
use crate::database::pool::DBLitePoolResult;
use crate::database::raw_conn_pool::RawPoolManager;

pub mod builder;

#[derive(Debug)]
pub struct DBClient {
    pub connection: DBLitePool,

    pub musicbrainz_client: Arc<MusicBrainzClient>,
    pub listenbrainz_client: Arc<ListenbrainzClient>,
}

impl DBClient {
    pub fn builder() -> ClientBuilder<(), (), ()> {
        ClientBuilder::default()
    }

    pub async fn get_connection(&self) -> DBLitePoolResult {
        self.connection.get().await
    }
    pub async fn get_raw_connection(
        &self,
    ) -> Result<Object<RawPoolManager>, PoolError<sqlx::Error>> {
        self.connection.get_raw_connection().await
    }

    pub async fn get_raw_connection_as_task(
        self: Arc<Self>,
    ) -> Result<Object<RawPoolManager>, PoolError<sqlx::Error>> {
        tokio::spawn(async move { self.connection.get_raw_connection().await })
            .await
            .unwrap()
    }

    #[cfg(test)]
    /// Create an in memory database with the default MB client
    pub async fn connect_in_memory_and_create() -> Result<Self, crate::Error> {
        Ok(Self::builder()
            .in_memory()
            .set_default_mb_client()
            .set_default_lb_client()
            .connect_and_migrate(1)
            .await?
            .build())
    }
}
