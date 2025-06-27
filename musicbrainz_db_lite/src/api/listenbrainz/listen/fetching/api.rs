use core::fmt::Write as _;

use listenbrainz::raw::response::UserListensResponse;

use crate::DBClient;

pub(super) async fn fetch_user_listens(
    crate_client: &DBClient,
    user: &str,
    min_ts: Option<i64>,
    max_ts: Option<i64>,
    count: Option<u32>,
) -> Result<UserListensResponse, crate::Error> {
    let mut url = format!(
        "{api_root}user/{username}/listens?count={count}",
        api_root = crate_client.listenbrainz_client.api_url(),
        username = user,
        count = count.unwrap_or(25)
    );

    if let Some(min_ts) = min_ts {
        write!(&mut url, "&min_ts={min_ts}").unwrap();
    }

    if let Some(max_ts) = max_ts {
        write!(&mut url, "&max_ts={max_ts}").unwrap();
    }

    let body = reqwest::get(url).await?.text().await?;
    serde_json::from_str(&body).map_err(|_| crate::Error::ListenFetchingError(body))
}
