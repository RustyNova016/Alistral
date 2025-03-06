use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use extend::ext;
use futures::TryFutureExt;
use sqlx::sqlite::SqliteConnectOptions;

use crate::database::db_connection::DbConnection;
use crate::database::raw_conn_pool::RawConnectionPool;
use crate::database::raw_conn_pool::RawPoolManager;

#[derive(Debug)]
pub struct PoolManager {
    raw_connections: RawConnectionPool,
}

impl PoolManager {
    pub fn create_pool(config: SqliteConnectOptions) -> DBLitePool {
        let raw_connections = RawPoolManager::create_pool(config);
        DBLitePool::builder(PoolManager { raw_connections })
            .build()
            .unwrap()
    }
}

impl managed::Manager for PoolManager {
    type Type = DbConnection;
    type Error = crate::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let conn = self.raw_connections.get().await?;
        Ok(DbConnection::new(
            deadpool::managed::Object::<RawPoolManager>::take(conn),
        ))
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        Ok(conn.ping().map_err(crate::Error::from).await?)
    }
}

/// The database pool with sync protected connections
pub type DBLitePool = managed::Pool<PoolManager>;

pub type DBLitePoolError = PoolError<crate::Error>;

pub type DBLitePoolResult = Result<Object<PoolManager>, DBLitePoolError>;

#[ext]
pub impl DBLitePool {
    #[allow(async_fn_in_trait)]
    async fn get_raw_connection(&self) -> Result<Object<RawPoolManager>, PoolError<sqlx::Error>> {
        self.manager().raw_connections.get().await
    }
}
