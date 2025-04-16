use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;

use crate::SymphonyzeClient;

pub fn formater(client: &SymphonyzeClient) -> MusicbrainzFormater {
    MusicbrainzFormater {
        artist_credits: true,
        listenbrainz_link: false,
        client: &client.mb_database,
    }
}
