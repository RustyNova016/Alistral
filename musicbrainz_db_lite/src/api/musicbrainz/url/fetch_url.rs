use core::fmt::Write as _;
use std::sync::Arc;

use futures::StreamExt as _;
use futures::stream;
use musicbrainz_rs::ApiRequest;
use serde::Deserialize;
use serde::Serialize;
use streamies::TryStreamies;

use crate::DBClient;
use crate::MBUrl;
use crate::Url;

impl Url {
    pub async fn fetch_by_ressource(
        client: &DBClient,
        url: &str,
    ) -> Result<MBUrl, musicbrainz_rs::Error> {
        let req = ApiRequest::new(format!(
            "https://musicbrainz.org/ws/2/url?resource={url}&fmt=json&inc=artist-rels+label-rels+release-group-rels+release-rels+recording-rels+url-rels+work-rels"
        ));

        req.get(&client.musicbrainz_client).await
    }

    pub async fn fetch_by_ressource_bulk(
        client: &DBClient,
        urls: Vec<&str>,
    ) -> Result<MultiUrlResponse, musicbrainz_rs::Error> {
        let mut req =  "https://musicbrainz.org/ws/2/url?fmt=json&inc=artist-rels+label-rels+release-group-rels+release-rels+recording-rels+url-rels+work-rels".to_string();

        for url in urls {
            write!(&mut req, "&resource={url}").unwrap()
        }

        ApiRequest::new(req).get(&client.musicbrainz_client).await
    }

    pub async fn fetch_and_save_by_ressource_as_task(
        client: Arc<DBClient>,
        url: &str,
    ) -> Result<Option<Url>, crate::Error> {
        let res = match Self::fetch_by_ressource(&client, url).await {
            Ok(val) => val,
            Err(err) => match err {
                musicbrainz_rs::Error::NotFound(_) => return Ok(None),
                _ => return Err(err.into()),
            },
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
            Err(err) => match err {
                musicbrainz_rs::Error::NotFound(_) => return Ok(Vec::new()),
                _ => return Err(err.into()),
            },
        };

        stream::iter(res.urls)
            .map(|url| Url::save_api_response_as_task(client.clone(), url))
            .buffer_unordered(8)
            .try_collect_vec()
            .await
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MultiUrlResponse {
    pub urls: Vec<MBUrl>,
}
