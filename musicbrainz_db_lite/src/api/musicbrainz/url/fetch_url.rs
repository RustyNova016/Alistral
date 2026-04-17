use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use musicbrainz_rs::ParsingError;
use musicbrainz_rs::api_bindium::ApiRequestError;
use musicbrainz_rs::api_bindium::endpoints::UriBuilderError;
use musicbrainz_rs::entity::url::MultiUrlResponse;
use snafu::ResultExt;
use snafu::Snafu;
use streamies::TryStreamies;

use crate::DBClient;
use crate::MBUrl;
use crate::Url;

impl Url {
    pub async fn fetch_by_ressource(
        client: &DBClient,
        url: &str,
    ) -> Result<Option<MBUrl>, UrlFetchingError> {
        let mut res = Self::fetch_by_ressource_bulk(client, vec![url]).await?;
        debug_assert!(res.urls.len() == 1);
        Ok(res.urls.pop())
    }

    pub async fn fetch_by_ressource_bulk(
        client: &DBClient,
        urls: Vec<&str>,
    ) -> Result<MultiUrlResponse, UrlFetchingError> {
        client
            .musicbrainz_client
            .endpoints()
            .ws_2_url()
            .ressources(urls)
            .artist_rels(true)
            .label_rels(true)
            .recording_rels(true)
            .release_group_rels(true)
            .release_rels(true)
            .url_rels(true)
            .work_rels(true)
            .call()
            .context(UriBuiderSnafu)?
            .send_async(&client.musicbrainz_client.api_client)
            .await
            .context(ApiSnafu)?
            .parse()
            .context(ParsingSnafu)
    }

    pub async fn fetch_and_save_by_ressource_as_task(
        client: Arc<DBClient>,
        url: &str,
    ) -> Result<Option<Url>, crate::Error> {
        let res = match Self::fetch_by_ressource(&client, url).await {
            Ok(Some(val)) => val,
            Ok(None) => return Ok(None),
            Err(err) => {
                return Err(err.into());
            }
        };

        Ok(Some(
            Url::save_api_response(&mut *client.get_raw_connection().await?, res).await?,
        ))
    }

    pub async fn fetch_and_save_by_ressource_bulk_as_task(
        client: Arc<DBClient>,
        urls: Vec<&str>,
    ) -> Result<Vec<Url>, crate::Error> {
        let res = match Self::fetch_by_ressource_bulk(&client, urls).await {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        };

        stream::iter(res.urls)
            .map(|url| Url::save_api_response_as_task(client.clone(), url))
            .buffer_unordered(8)
            .try_collect_vec()
            .await
    }
}

#[derive(Debug, Snafu)]
pub enum UrlFetchingError {
    UriBuider {
        source: UriBuilderError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    ApiError {
        source: ApiRequestError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    ParsingError {
        source: ParsingError,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}
