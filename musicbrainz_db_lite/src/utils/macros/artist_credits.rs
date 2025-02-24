macro_rules! impl_artist_credits {
    ($row_struct: ty, $entity_table_name: expr) => {
        impl $row_struct {
            pub async fn get_artist_credits(
                &self,
                conn: &mut sqlx::SqliteConnection,
            ) -> Option<Result<crate::models::musicbrainz::artist_credit::ArtistCredits, sqlx::Error>> {

                match self.artist_credit {
                    Some(id) => Some(crate::models::musicbrainz::artist_credit::ArtistCredits::find_by_id(conn, id).await),
                    None => None,
                }
            }

            pub async fn get_artist_credits_or_fetch(
                &self,
                conn: &mut sqlx::SqliteConnection,
                client: &crate::DBClient
            ) -> Result<crate::models::musicbrainz::artist_credit::ArtistCredits, crate::Error> {

                match self.artist_credit {
                    Some(id) => Ok(crate::models::musicbrainz::artist_credit::ArtistCredits::find_by_id(conn, id).await?),
                    None => {
                        let new_self = self.refetch(conn, client).await?;
                        Ok(crate::models::musicbrainz::artist_credit::ArtistCredits::find_by_id(conn, new_self.artist_credit.expect("The artist creadits should be loaded after fetching")).await?)
                    },
                }
            }

            pub async fn set_artist_credits(
                &mut self,
                conn: &mut sqlx::SqliteConnection,
                credits_id: i64,
            ) -> Result<(), sqlx::Error> {
                sqlx::query(&format!("UPDATE {} SET artist_credit = ? WHERE id = ?", $entity_table_name))
                .bind(credits_id)
                .bind(self.id)
                .execute(conn)
                .await?;

                self.artist_credit = Some(credits_id);

                Ok(())
            }

            /// Return a string containing the recording name and its artist credits
            ///
            /// Exemple: Never Gonna Give You Up by Rick Astley
            pub async fn format_with_credits(
                &self,
                conn: &mut sqlx::SqliteConnection,
                client: &crate::DBClient
            ) -> Result<String, crate::Error> {
                let credit = self.get_artist_credits_or_fetch(conn, client).await?.to_string();
                Ok(format!("{} by {}", self.title, credit))
            }
        }
    };
}

pub(crate) use impl_artist_credits;
