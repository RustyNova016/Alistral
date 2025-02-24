macro_rules! impl_get_and_fetch {
    ($row_struct: ty) => {
        impl $row_struct {
            /// Get the entity from its MBID, and if it isn't cached in the database, fetch it
            pub async fn get_or_fetch(
                conn: &mut sqlx::SqliteConnection,
                client: &crate::DBClient,
                mbid: &str,
            ) -> Result<Option<Self>, crate::Error> {
                let data = Self::find_by_mbid(conn, mbid).await?;

                match data {
                    Some(val) => Ok(Some(val)),
                    None => Self::fetch_and_save(conn, client, mbid).await,
                }
            }
        }
    };
}

pub(crate) use impl_get_and_fetch;
