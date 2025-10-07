use std::sync::Arc;

use sequelles::has_rowid::HasRowID;

use crate::ArtistCredits;
use crate::FetchAndSave;
use crate::HasMBID;
use crate::models::shared_traits::has_table::HasTable;

/// Trait for all the entities that have artist credits
pub trait HasArtistCredits<U>
where
    Self: FetchAndSave<U> + HasMBID + Send + Sync + HasRowID + HasTable,
    U: Send,
{
    /// Retrieve the artist creditd id of the current entity
    fn get_artist_credits_id(&self) -> Option<&i64>;

    /// Return the title of the entity
    fn get_title(&self) -> &str;

    /// Set the id of the artist credits associated to this entity
    fn set_artist_credits_id(&mut self, id: Option<i64>);

    /// Get the artist credits of the entity or refetch the entity
    fn get_artist_credits_or_fetch(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<ArtistCredits, crate::Error>> + Send {
        async {
            match self.get_artist_credits_id() {
                Some(id) => Ok(ArtistCredits::find_by_id(conn, *id).await?),

                None => {
                    let new_self = self.refetch_with_conn(conn, client).await?;
                    let new_id = new_self
                        .get_artist_credits_id()
                        .expect("The artist credits should be loaded after fetching");

                    Ok(ArtistCredits::find_by_id(conn, *new_id).await?)
                }
            }
        }
    }

    /// Get the artist credits of the entity or refetch the entity. Uses a task for it
    fn get_artist_credits_or_fetch_tasked(
        &self,
        client: Arc<crate::DBClient>,
    ) -> impl Future<Output = Result<ArtistCredits, crate::Error>> + Send
    where
        Self: 'static,
        U: 'static,
    {
        async move {
            match self.get_artist_credits_id() {
                Some(id) => {
                    Ok(ArtistCredits::find_by_id(&mut *client.get_conn().await?, *id).await?)
                }

                None => {
                    let new_self = self.refetch_as_task(client.clone()).await?;
                    let new_id = new_self
                        .get_artist_credits_id()
                        .expect("The artist credits should be loaded after fetching");

                    Ok(ArtistCredits::find_by_id(&mut *client.get_conn().await?, *new_id).await?)
                }
            }
        }
    }

    /// Return a string containing the recording name and its artist credits
    ///
    /// Exemple: Never Gonna Give You Up by Rick Astley
    fn format_with_credits(
        &self,
        conn: &mut sqlx::SqliteConnection,
        client: &crate::DBClient,
    ) -> impl std::future::Future<Output = Result<String, crate::Error>> + Send {
        async {
            let credit = self
                .get_artist_credits_or_fetch(conn, client)
                .await?
                .to_string();
            Ok(format!("{} by {}", self.get_title(), credit))
        }
    }

    fn set_artist_credits(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        credits_id: i64,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
            sqlx::query(&format!(
                "UPDATE `{}` SET `artist_credit` = $1 WHERE `id` = $2",
                Self::TABLE_NAME
            ))
            .bind(credits_id)
            .bind(self.rowid())
            .execute(conn)
            .await?;

            self.set_artist_credits_id(Some(credits_id));

            Ok(())
        }
    }
}
