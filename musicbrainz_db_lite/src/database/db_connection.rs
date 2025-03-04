use std::sync::Arc;

use async_fn_stream::try_fn_stream;
use futures::TryStreamExt as _;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::Connection as _;
use sqlx::Executor;
use sqlx::Sqlite;
use sqlx::SqliteConnection;
use tokio::sync::Mutex;
use tokio::sync::OwnedMutexGuard;

/// A wrapper arround sqlx's Sqlite connection.
///
/// Sqlite can only accept one write transaction at the time.
/// If multiple write transations are requested, they will be run one after another, in a first in, last out order.
///
/// This struct allow to change this order, by providing a first in, first out `Mutex`. This also prevent hitting write timeouts.
/// This doesn't cover other application using the database at the same time, but it's better than nothing
#[derive(Debug, Clone)]
pub struct DbConnection(Arc<Mutex<sqlx::SqliteConnection>>);

impl DbConnection {
    pub fn new(connection: sqlx::SqliteConnection) -> Self {
        Self(Arc::new(Mutex::new(connection)))
    }

    pub async fn acquire_guarded(&self) -> tokio::sync::MutexGuard<'_, sqlx::SqliteConnection> {
        self.0.lock().await
    }

    pub async fn connect_with(config: &SqliteConnectOptions) -> Result<Self, sqlx::Error> {
        let inner = SqliteConnection::connect_with(config).await?;
        Ok(Self::new(inner))
    }

    pub async fn ping(&self) -> Result<(), sqlx::Error> {
        self.0.lock().await.ping().await
    }

    pub async fn take(self) -> OwnedMutexGuard<sqlx::SqliteConnection> {
        self.0.lock_owned().await
    }
}

impl<'c> Executor<'c> for DbConnection {
    type Database = Sqlite;

    fn fetch_many<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> futures::stream::BoxStream<
        'e,
        Result<
            sqlx::Either<
                <Self::Database as sqlx::Database>::QueryResult,
                <Self::Database as sqlx::Database>::Row,
            >,
            sqlx::Error,
        >,
    >
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        Box::pin(try_fn_stream(|emitter| async move {
            let mut conn = self.0.lock().await;
            let mut s = conn.fetch_many(query);

            while let Some(v) = s.try_next().await? {
                emitter.emit(v).await;
            }

            Ok(())
        }))
    }

    fn fetch_optional<'e, 'q: 'e, E>(
        self,
        query: E,
    ) -> futures::future::BoxFuture<
        'e,
        Result<Option<<Self::Database as sqlx::Database>::Row>, sqlx::Error>,
    >
    where
        'c: 'e,
        E: 'q + sqlx::Execute<'q, Self::Database>,
    {
        Box::pin(async move { self.0.lock().await.fetch_optional(query).await })
    }

    fn prepare_with<'e, 'q: 'e>(
        self,
        sql: &'q str,
        parameters: &'e [<Self::Database as sqlx::Database>::TypeInfo],
    ) -> futures::future::BoxFuture<
        'e,
        Result<<Self::Database as sqlx::Database>::Statement<'q>, sqlx::Error>,
    >
    where
        'c: 'e,
    {
        Box::pin(async move { self.0.lock().await.prepare_with(sql, parameters).await })
    }

    fn describe<'e, 'q: 'e>(
        self,
        sql: &'q str,
    ) -> futures::future::BoxFuture<'e, Result<sqlx::Describe<Self::Database>, sqlx::Error>>
    where
        'c: 'e,
    {
        Box::pin(async move { self.0.lock().await.describe(sql).await })
    }
}
