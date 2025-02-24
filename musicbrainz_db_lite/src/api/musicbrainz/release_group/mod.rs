use musicbrainz_rs_nova::entity::release_group::ReleaseGroup as MBReleaseGroup;

use crate::models::musicbrainz::artist_credit::ArtistCredits;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::musicbrainz::tags::Tag;
use crate::utils::date_utils::date_to_timestamp;
use crate::Error;

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

    /// Merge an Entity with its counterpart in musicbrainz_rs_nova. It always prefers data from musicbrainz_rs_nova over the cached one
    pub fn merge_api_data(self, new: MBReleaseGroup) -> Self {
        Self {
            id: self.id,
            mbid: new.id,
            title: new.title,
            annotation: new.annotation.or(self.annotation),
            disambiguation: new.disambiguation,
            first_release_date: new
                .first_release_date
                .map(|date| date_to_timestamp(date).unwrap())
                .or(self.first_release_date),
            primary_type_id: new.primary_type_id.or(self.primary_type_id),
            full_update_date: self.full_update_date,
            artist_credit: None,
        }
    }

    /// Save the responce from `musicbrainz_rs_nova` and its children relations
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
