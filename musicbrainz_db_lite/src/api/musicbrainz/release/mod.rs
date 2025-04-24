use musicbrainz_rs_nova::entity::release::Release as MBRelease;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::artist_credit::ArtistCredits;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::release::LabelInfo;
use crate::models::musicbrainz::release::Media;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::release_group::ReleaseGroup;
use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::save_from::SaveFrom;
use crate::utils::date_utils::date_to_timestamp;
use crate::Error;

pub mod fetching;
pub mod label_info;
pub mod media;
pub mod tracks;

impl Release {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBRelease,
    ) -> Result<Self, crate::Error> {
        Release::add_redirect_mbid(conn, &value.id).await?;
        Release::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Release::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBRelease) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            mbid: new.id,
            artist_credit: self.artist_credit,
            barcode: new.barcode.or(self.barcode),
            country: new.country.or(self.country),
            date: new
                .date
                .map(|date| date_to_timestamp(date).unwrap())
                .or(self.date),
            disambiguation: new.disambiguation.or(self.disambiguation),
            packaging: self.packaging, //TODO: Packaging to string
            title: new.title,
            quality: self.quality, //TODO: Quality to string
            status: self.status,   //TODO: Status to string
            full_update_date: self.full_update_date,
            release_group: self.release_group,
        }
    }

    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBRelease,
    ) -> Result<Self, crate::Error> {
        let mut new_value = Release::save_api_response(conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            new_value.set_artist_credits(conn, credits.0).await?;
        }

        if let Some(values) = value.media.clone() {
            Media::save_api_response(conn, values, new_value.id).await?;
        }

        if let Some(values) = value.label_info {
            LabelInfo::save_api_response(conn, values, new_value.id).await?;
        }

        if let Some(release_group) = value.release_group.clone() {
            let release_group = ReleaseGroup::save_api_response(conn, release_group).await?;
            new_value.release_group = Some(release_group.id);
            new_value.upsert(conn).await?;
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

impl FetchAndSave<MBRelease> for Release {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::set_redirection(conn, mbid, id).await
    }

    async fn set_full_update(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        self.reset_full_update_date(conn).await
    }
}

impl SaveFrom<MBRelease> for Release {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBRelease,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}
