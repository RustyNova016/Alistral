use musicbrainz_rs::entity::release_group::ReleaseGroup as MBReleaseGroup;

use crate::Error;
use crate::MBIDRedirection as _;
use crate::models::musicbrainz::artist_credit::ArtistCredits;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::save_from::SaveFrom;
use crate::utils::date_utils::date_string_to_timestamp;

pub mod fetching;

impl ReleaseGroup {
    /// Save an api response into the database
    pub async fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBReleaseGroup,
    ) -> Result<Self, crate::Error> {
        Self::add_redirect_mbid(conn, &value.id).await?;
        Self::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Self::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    /// Merge an Entity with its counterpart in musicbrainz_rs. It always prefers data from musicbrainz_rs over the cached one
    pub fn merge_api_data(self, new: MBReleaseGroup) -> Self {
        Self {
            id: self.id,
            mbid: new.id,
            title: new.title,
            annotation: new.annotation.or(self.annotation),
            disambiguation: new.disambiguation,
            first_release_date: new
                .first_release_date
                .and_then(|date| date_string_to_timestamp(date).or(self.first_release_date)),
            primary_type_id: new.primary_type_id.or(self.primary_type_id),
            full_update_date: self.full_update_date,
            artist_credit: None,
        }
    }

    /// Save the responce from `musicbrainz_rs` and its children relations
    pub async fn save_api_response_recursive(
        conn: &mut sqlx::SqliteConnection,
        value: MBReleaseGroup,
    ) -> Result<Self, crate::Error> {
        let mut new_value = Self::save_api_response(conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            new_value.set_artist_credits(conn, credits.0).await?;
        }

        if let Some(releases) = value.releases.clone() {
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

impl FetchAndSave<MBReleaseGroup> for ReleaseGroup {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::link_mbid(conn, mbid, id).await
    }
}

impl CompletenessFlag for ReleaseGroup {
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

impl SaveFrom<MBReleaseGroup> for ReleaseGroup {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBReleaseGroup,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}
