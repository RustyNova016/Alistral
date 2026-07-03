use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsyncDyn;

use crate::utils::constants::LISTENBRAINZ_FMT;

pub struct EntityNameCell<T> {
    pub entity: T,
}

impl<T> EntityNameCell<T>
where
    T: FormatWithAsyncDyn<MusicbrainzFormater, Error = musicbrainz_db_lite::Error>,
{
    pub async fn format(&self) -> String {
        self.entity.format_with_async(&LISTENBRAINZ_FMT)
            .await
            .unwrap()
    }
}
