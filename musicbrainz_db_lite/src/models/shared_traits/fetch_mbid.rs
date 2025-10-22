use std::sync::Arc;

use crate::DBClient;

pub trait FetchMBID<T> {
    fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<T, musicbrainz_rs::GetRequestError>> + Send;

    fn fetch_from_mbid_as_task(
        client: Arc<DBClient>,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<T, musicbrainz_rs::GetRequestError>> + Send
    where
        T: Send + 'static,
    {
        async {
            let mbid = mbid.to_owned();

            tokio::spawn(async move { Self::fetch_from_mbid(client.as_ref(), &mbid).await })
                .await
                .unwrap()
        }
    }
}
