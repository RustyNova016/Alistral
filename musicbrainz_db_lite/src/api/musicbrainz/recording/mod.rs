pub mod fetching;
use crate::models::musicbrainz::genre::genre_tag::GenreTag;
use crate::models::musicbrainz::tags::Tag;
use crate::Error;
use crate::{
    api::SaveToDatabase,
    models::musicbrainz::{
        artist_credit::ArtistCredits,
        recording::Recording,
        release::{Release, Track},
    },
    utils::date_utils::date_to_timestamp,
};
use musicbrainz_rs_nova::entity::recording::Recording as MBRecording;
use musicbrainz_rs_nova::entity::release::Release as MBRelease;
use sqlx::SqliteConnection;

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
                .map(|date| date_to_timestamp(date).unwrap())
                .or(self.first_release_date),
        }
    }

    /// Save a recording from the api data. It also save the relationships.
    pub async fn save_api_response_recursive(
        conn: &mut SqliteConnection,
        value: MBRecording,
    ) -> Result<Self, crate::Error> {
        // Save the recording
        let mut new_value = Recording::save_api_response(&mut *conn, value.clone()).await?;

        // Save relations
        if let Some(artist_credits) = value.artist_credit.clone() {
            let credits = ArtistCredits::save_api_response(conn, artist_credits).await?;
            new_value.set_artist_credits(conn, credits.0).await?;
        }

        if let Some(releases) = value.releases.clone() {
            for release in releases {
                let gids = get_track_gids_from_release(release.clone());
                Release::save_api_response_recursive(conn, release).await?;

                for gid in gids {
                    //TODO: Improve flow to prevent updating after insert, thus making `tracks`.`recording` non optional
                    Track::set_recording_id_from_gid(conn, new_value.id, &gid).await?;
                }
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

impl SaveToDatabase for MBRecording {
    type ReturnedData = Recording;

    async fn save(self, conn: &mut SqliteConnection) -> Result<Self::ReturnedData, crate::Error> {
        Recording::save_api_response_recursive(conn, self).await
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
