use musicbrainz_rs_nova::entity::release::Media as MBMedia;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::release::{Media, Track};

impl Media {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: Vec<MBMedia>,
        release_id: i64,
    ) -> Result<Vec<Self>, crate::Error> {
        let mut converteds = Vec::new();
        for media in value {
            let new_media = Media {
                id: Default::default(),
                disc_count: media.disc_count.map(|n| n as i64),
                format: media.format,
                position: media.position.map(|n| n as i64),
                title: media.title,
                release: release_id,
                track_count: media.track_count as i64,
                track_offset: media.track_offset.map(|n| n as i64),
            };

            let new_media = new_media.upsert(&mut *conn).await?;

            if let Some(tracks) = media.tracks {
                Track::save_api_responses(&mut *conn, tracks, new_media.id).await?;
            }

            converteds.push(new_media);
        }

        Ok(converteds)
    }
}
