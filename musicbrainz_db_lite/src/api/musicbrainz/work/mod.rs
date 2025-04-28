pub mod fetching;
use musicbrainz_rs_nova::entity::work::Work as MBWork;

use crate::Error;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::tags::Tag;
use crate::models::musicbrainz::work::Work;
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::save_from::SaveFrom;
use crate::utils::strip_quotes;

impl Work {
    /// Save an api response into the database
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBWork,
    ) -> Result<Self, crate::Error> {
        Self::add_redirect_mbid(conn, &value.id).await?;
        Self::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Self::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    /// Merge an Entity with its counterpart in musicbrainz_rs_nova. It always prefers data from musicbrainz_rs_nova over the cached one
    pub fn merge_api_data(self, new: MBWork) -> Self {
        Self {
            id: self.id,
            mbid: new.id,
            title: new.title,
            annotation: new.annotation.or(self.annotation),
            disambiguation: new.disambiguation.or(self.disambiguation),
            work_type: new
                .work_type
                .map(|w| {
                    strip_quotes(
                        serde_json::to_string(&w).expect("The enum should be serializable"),
                    )
                })
                .or(self.work_type),
            full_update_date: self.full_update_date,
        }
    }

    /// Save the responce from `musicbrainz_rs_nova` and its children relations
    pub async fn save_api_response_recursive(
        conn: &mut sqlx::SqliteConnection,
        value: MBWork,
    ) -> Result<Self, crate::Error> {
        let new_value = Self::save_api_response(conn, value.clone()).await?;

        // Save relations

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

impl FetchAndSave<MBWork> for Work {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::set_redirection(conn, mbid, id).await
    }
}

impl CompletenessFlag for Work {
    async fn set_full_update(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        self.reset_full_update_date(conn).await
    }

    fn is_complete(&self) -> bool {
        self.full_update_date.is_some()
    }
}

impl SaveFrom<MBWork> for Work {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBWork,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}
