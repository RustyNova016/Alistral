use musicbrainz_rs::entity::label::Label as MBLabel;
use sqlx::SqliteConnection;

use crate::Error;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::tags::Tag;
use crate::models::musicbrainz::{label::Label, release::Release};
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::mbid_redirection::MBIDRedirection;
use crate::models::shared_traits::save_from::SaveFrom;

pub mod fetching;

impl Label {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBLabel,
    ) -> Result<Self, crate::Error> {
        Self::add_redirect_mbid(conn, &value.id).await?;
        Self::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Self::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBLabel) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            country: new.country.or(self.country),
            disambiguation: new.disambiguation.or(self.disambiguation),
            full_update_date: self.full_update_date,
            label_code: new.label_code.map(|v| v as i64).or(self.label_code),
            label_type: new
                .label_type
                .map(|v| serde_json::to_string(&v).unwrap())
                .or(self.label_type),
            mbid: new.id,
            name: new.name,
            sort_name: new.sort_name.or(self.sort_name),
        }
    }

    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBLabel,
    ) -> Result<Self, crate::Error> {
        let new_value = Self::save_api_response(conn, value.clone()).await?;

        // Save relations
        if let Some(releases) = value.releases {
            for release in releases {
                Release::save_api_response(conn, release).await?;
            }
        }

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

impl FetchAndSave<MBLabel> for Label {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::link_mbid(conn, mbid, id).await
    }
}

impl CompletenessFlag for Label {
    async fn set_full_update(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        let ts = chrono::Utc::now().timestamp();
        sqlx::query!(
            "UPDATE `labels` SET `full_update_date` = $1 WHERE id = $2",
            ts,
            self.id
        )
        .execute(conn)
        .await?;
        Ok(())
    }

    fn is_complete(&self) -> bool {
        self.full_update_date.is_some()
    }
}

impl SaveFrom<MBLabel> for Label {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBLabel,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}
