use musicbrainz_rs_nova::entity::artist_credit::ArtistCredit as MBArtistCredit;
use sqlx::SqliteConnection;

use crate::models::musicbrainz::{
    artist::Artist,
    artist_credit::{ArtistCredit, ArtistCredits},
};

impl ArtistCredits {
    pub async fn save_api_response(
        conn: &mut SqliteConnection,
        value: Vec<MBArtistCredit>,
    ) -> Result<ArtistCredits, crate::Error> {
        let mut rows = Vec::new();

        for (position, resp) in value.into_iter().enumerate() {
            rows.push(ArtistCredit {
                artist_credit: Default::default(),
                artist_gid: resp.artist.id.clone(),
                join_phrase: resp.joinphrase.unwrap_or("".to_string()),
                name: resp.name,
                position: position as i64,
            });

            Artist::save_api_response_recursive(conn, resp.artist).await?;
        }

        Ok(ArtistCredits::save(conn, &rows).await?)
    }
}
