use proc_macro2::TokenStream;
use syn::Ident;

pub fn impl_update_date(struct_name: &Ident, table_name: &str, pk: &str) -> TokenStream {
    let sql = format!(
        "UPDATE `{}` SET `full_update_date` = ? WHERE {} = ?",
        table_name, pk
    );

    let is_fully_fetched_doc = format!("Return true if the {} is fully fetched.", struct_name);

    quote::quote! {
        /// Reset the full update date to be now
        pub async fn reset_full_update_date(&mut self, conn: &mut sqlx::SqliteConnection) -> Result<(), sqlx::Error> {
            let ts = chrono::Utc::now().timestamp();
            sqlx::query(#sql).bind(ts).bind(self.id).execute(conn).await?;
            self.full_update_date = Some(ts);
            Ok(())
        }

        /// Get from the database and perform an update if the data isn't fully present
        pub async fn get_or_fetch_as_complete(&self, conn: &mut sqlx::SqliteConnection, client: &crate::DBClient) -> Result<Self, crate::Error> {
            match self.full_update_date {
                Some(_) => Ok(self.clone()),
                None => self.refetch(conn, client).await
            }
        }


        #[doc = #is_fully_fetched_doc]
        pub fn is_fully_fetched(&self) -> bool {
            self.full_update_date.is_some()
        }

        /// Assert that a list of recordings are fetched, or panic
        pub fn assert_recordings_fetched(vals: &[Self]) {
            for val in vals {
                if !val.is_fully_fetched() {
                    panic!("Recording {} isn't fully fetched", val.id)
                }
            }
        }

        pub async fn fetch_if_incomplete(
            &self,
            conn: &mut sqlx::SqliteConnection,
            client: &crate::DBClient
        ) -> Result<(), crate::Error> {
            if self.full_update_date.is_none() {
                self.refetch(conn, client).await?;
            }
            Ok(())
        }

        /// Get from the database and perform an update if the data isn't fully present
        pub async fn get_or_fetch_as_complete_from_mbid(conn: &mut sqlx::SqliteConnection, client: &crate::DBClient, mbid: &str) -> Result<Option<Self>, crate::Error> {
            match Self::find_by_mbid(conn, mbid).await? {
                Some(data) => {
                    if data.full_update_date.is_none() {
                        return Ok(Some(data.refetch(conn, client).await?))
                    }
                    Ok(Some(data))
                },
                None => Self::fetch_and_save(conn, client, mbid).await
            }
        }

        /// Refresh the data in the database by refetching the entity
        pub async fn refetch(&self, conn: &mut sqlx::SqliteConnection, client: &crate::DBClient) -> Result<Self, crate::Error> {
            Self::fetch_and_save(conn, client, &self.mbid).await?.ok_or(crate::Error::UnknownUpstream(self.mbid.clone()))
        }

        /// Refetch the entity and replace the inner values with the new ones
        pub async fn refetch_and_load(&mut self, conn: &mut sqlx::SqliteConnection, client: &crate::DBClient) -> Result<(), crate::Error> {
            *self = self.refetch(conn, client).await?;

            Ok(())
        }
    }
}
