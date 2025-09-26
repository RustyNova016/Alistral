use musicbrainz_rs::entity::artist::Artist as MBArtist;
use sqlx::SqliteConnection;

use crate::Error;
use crate::MBIDRedirection;
use crate::models::musicbrainz::artist::Artist;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::save_from::SaveFrom;

pub mod browse;
pub mod fetching;

impl Artist {
    pub fn merge_api_data(self, new: MBArtist) -> Self {
        Self {
            annotation: new.annotation.or(self.annotation),
            id: self.id,
            country: new.country.or(self.country),
            disambiguation: new.disambiguation,
            mbid: new.id,
            name: new.name,
            sort_name: new.sort_name,
            full_update_date: self.full_update_date,
        }
    }

    #[tracing::instrument]
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, crate::Error> {
        Artist::add_redirect_mbid(conn, &value.id).await?;
        Artist::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Artist::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    /// Save a recording from the api data. It also save the relationships
    #[tracing::instrument]
    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, crate::Error> {
        let new_value = Artist::save_api_response(&mut *conn, value.clone()).await?;

        if let Some(relations) = value.relations {
            // Remove all the old relations
            new_value.delete_all_relations(conn).await?;
            for rel in relations {
                match new_value.save_relation(conn, rel).await {
                    Ok(_) => {}
                    Err(Error::RelationNotImplemented) => {}
                    Err(err) => {
                        Err(err)?;
                    }
                }
            }
        }

        if let Some(tags) = value.tags {
            for tag in tags {
                Tag::save_api_response::<Self>(conn, tag, &new_value).await?;
            }
        }

        if let Some(genres) = value.genres {
            for genre in genres {
                GenreTag::save_api_response::<Self>(conn, genre, &new_value).await?;
            }
        }

        Ok(new_value)
    }
}

impl FetchAndSave<MBArtist> for Artist {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::link_mbid(conn, mbid, id).await
    }
}

impl CompletenessFlag for Artist {
    async fn set_full_update(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        let ts = chrono::Utc::now().timestamp();
        sqlx::query!(
            "UPDATE `artists` SET `full_update_date` = $1 WHERE id = $2",
            ts,
            self.id
        )
        .execute(conn)
        .await?;

        self.full_update_date = Some(ts);
        Ok(())
    }

    fn is_complete(&self) -> bool {
        self.full_update_date.is_some()
    }
}

impl SaveFrom<MBArtist> for Artist {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBArtist,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}
