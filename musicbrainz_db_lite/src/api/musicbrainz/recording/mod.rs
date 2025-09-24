use musicbrainz_rs::entity::recording::Recording as MBRecording;
use musicbrainz_rs::entity::release::Release as MBRelease;
use sqlx::Acquire;
use sqlx::SqliteConnection;

use crate::Error;
use crate::HasArtistCredits as _;
use crate::Track;
use crate::models::musicbrainz::artist_credit::ArtistCredits;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::isrc::ISRC;
use crate::models::musicbrainz::recording::Recording;
use crate::models::musicbrainz::release::Release;
use crate::models::musicbrainz::tags::Tag;
use crate::models::shared_traits::completeness::CompletenessFlag;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::mbid_redirection::MBIDRedirection;
use crate::models::shared_traits::save_from::SaveFrom;
use crate::utils::date_utils::date_string_to_timestamp;

pub mod fetching;

impl Recording {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, crate::Error> {
        Recording::add_redirect_mbid(conn, &value.id).await?;
        Recording::find_by_mbid(conn, &value.id) // Get old data
            .await?
            .unwrap_or_else(Recording::default) // Or create new
            .merge_api_data(value.clone()) // Merge new data if it exists
            .upsert(conn) // Upsert the new data
            .await
    }

    pub fn merge_api_data(self, new: MBRecording) -> Self {
        Self {
            id: self.id,
            annotation: new.annotation.or(self.annotation),
            mbid: new.id,
            artist_credit: self.artist_credit,
            disambiguation: new.disambiguation.or(self.disambiguation),
            length: new.length.map(|n| n as i64).or(self.length),
            title: new.title,
            full_update_date: self.full_update_date,
            video: new.video.map(|n| n as i64).or(self.video),
            first_release_date: new
                .first_release_date
                .and_then(|date| date_string_to_timestamp(date).or(self.first_release_date)),
        }
    }

    /// Save a recording from the api data. It also save the relationships.
    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, crate::Error> {
        let mut conn = conn.begin().await?;

        // Save the recording
        let mut new_value = Recording::save_api_response(&mut conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(&mut conn, artist_credits).await?;
            new_value.set_artist_credits(&mut conn, credits.0).await?;
        }

        if let Some(releases) = value.releases.clone() {
            for release in releases {
                let gids = get_track_gids_from_release(release.clone());
                Release::save_api_response_recursive(&mut conn, release).await?;

                for gid in gids {
                    //TODO: Improve flow to prevent updating after insert, thus making `tracks`.`recording` non optional
                    Track::set_recording_id_from_gid(&mut conn, new_value.id, &gid).await?;
                }
            }
        }

        if let Some(relations) = value.relations {
            // Remove all the old relations
            new_value.delete_all_relations(&mut conn).await?;

            for rel in relations {
                match new_value.save_relation(&mut conn, rel).await {
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
                Tag::save_api_response::<Self>(&mut conn, tag, &new_value).await?;
            }
        }

        if let Some(genres) = value.genres {
            for genre in genres {
                GenreTag::save_api_response::<Self>(&mut conn, genre, &new_value).await?;
            }
        }

        if let Some(isrcs) = value.isrcs {
            ISRC::upsert_recording_isrcs_list(&mut conn, new_value.id, isrcs).await?;
        }

        conn.commit().await?;

        Ok(new_value)
    }
}

impl FetchAndSave<MBRecording> for Recording {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Self::link_mbid(conn, mbid, id).await
    }
}

impl CompletenessFlag for Recording {
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

impl SaveFrom<MBRecording> for Recording {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, crate::Error> {
        Self::save_api_response_recursive(conn, value).await
    }
}

fn get_track_gids_from_release(release: MBRelease) -> Vec<String> {
    let mut gids = Vec::new();

    for media in release.media.unwrap_or_default() {
        for track in media.tracks.unwrap_or_else(Vec::new) {
            gids.push(track.id);
        }
    }

    gids
}
