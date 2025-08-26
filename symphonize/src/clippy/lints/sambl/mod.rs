use musicbrainz_db_lite::Artist;
use musicbrainz_db_lite::Url;

use crate::SymphonyzeClient;
use crate::clippy::clippy_lint::MbClippyLint;
use crate::sambl::api_results::AlbumData;

pub mod missing_samble_release;

pub trait SamblLint: MbClippyLint {
    fn check_album_data(
        client: &SymphonyzeClient,
        artist: &Artist,
        album: &AlbumData,
    ) -> impl std::future::Future<Output = Result<Option<Self>, crate::Error>> + Send;

    fn refresh_album_data(
        client: &SymphonyzeClient,
        album: &AlbumData,
    ) -> impl std::future::Future<Output = Result<(), crate::Error>> + Send {
        async {
            Url::fetch_and_save_by_ressource_as_task(
                client.mb_database.clone(),
                &album.url,
            )
            .await?;

            Ok(())
        }
    }
}
