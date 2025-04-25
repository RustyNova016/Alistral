use crate::DBClient;

pub trait FetchMBID<T> {
    fn fetch_from_mbid(
        client: &DBClient,
        mbid: &str,
    ) -> impl std::future::Future<Output = Result<T, musicbrainz_rs_nova::Error>> + Send;
}
