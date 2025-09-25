use deadpool::managed;
use deadpool::managed::Object;
use deadpool::managed::PoolError;
use sqlx::Connection as _;
use sqlx::SqliteConnection;
use sqlx::sqlite::SqliteConnectOptions;

#[derive(Debug)]
pub struct DBLitePoolManager {
    config: SqliteConnectOptions,
}

impl DBLitePoolManager {
    pub fn create_pool(config: SqliteConnectOptions) -> DBLitePool {
        DBLitePool::builder(DBLitePoolManager { config })
            .build()
            .unwrap()
    }
}

impl managed::Manager for DBLitePoolManager {
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

/// A connection pool of raw `SqliteConnection`.
pub type DBLitePool = managed::Pool<DBLitePoolManager>;

pub type DBLitePoolError = PoolError<sqlx::Error>;

pub type DBLitePoolConn = Object<DBLitePoolManager>;

pub type DBLitePoolResult = Result<DBLitePoolConn, DBLitePoolError>;
