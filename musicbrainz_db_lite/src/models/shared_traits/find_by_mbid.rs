use core::ops::Deref;

use super::find_by::FindBy;

pub trait FindByMBID: Sized {
    fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        id: &str,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;

    
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
