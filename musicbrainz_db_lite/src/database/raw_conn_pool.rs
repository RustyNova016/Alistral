use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::Connection as _;
use sqlx::SqliteConnection;

#[derive(Debug)]
pub struct RawPoolManager {
    config: SqliteConnectOptions,
}

impl RawPoolManager {
    pub fn create_pool(config: SqliteConnectOptions) -> RawConnectionPool {
        RawConnectionPool::builder(RawPoolManager { config })
            .build()
            .unwrap()
    }
}

impl managed::Manager for RawPoolManager {
    type Type = sqlx::SqliteConnection;
    type Error = sqlx::Error;

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        SqliteConnection::connect_with(&self.config).await
    }

    async fn recycle(
        &self,
        conn: &mut Self::Type,
        _: &managed::Metrics,
    ) -> managed::RecycleResult<Self::Error> {
        Ok(conn.ping().await?)
    }
}

/// A connection pool of raw `SqliteConnection`. Prefer using [`DBLitePool`] instead
pub type RawConnectionPool = managed::Pool<RawPoolManager>;

pub type RawPoolError = PoolError<sqlx::Error>;

pub type RawPoolResult = Result<Object<RawPoolManager>, RawPoolError>;
