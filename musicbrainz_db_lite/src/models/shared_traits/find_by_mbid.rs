use core::ops::Deref;

use crate::DBClient;

use super::find_by::FindBy;

/// Find the current entity in the database using its MBID
pub trait FindByMBID: Sized {
    /// Find the current entity in the database using its MBID
    fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;

    /// Find the current entity in the database using its MBID
    ///
    /// This will use the client's connection pool
    fn find_by_mbid_with_pool(
        client: &DBClient,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send {
        async { Self::find_by_mbid(&mut *client.get_raw_connection().await?, id).await }
    }
}

pub struct MBID(String);

impl Deref for MBID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: FindByMBID> FindBy<MBID> for T {
    async fn find_by(
        conn: &mut sqlx::SqliteConnection,
        id: MBID,
    ) -> Result<Option<Self>, crate::Error> {
        Self::find_by_mbid(conn, &id).await
    }
}
