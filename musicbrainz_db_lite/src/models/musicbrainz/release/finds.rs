use std::sync::Arc;

use snafu::ResultExt as _;

use crate::DBClient;
use crate::MBIDRedirection;
use crate::Url;
use crate::error::sqlx_error::SqlxSnafu;
use crate::models::shared_traits::find_by_mbid::FindByMBID;
use crate::models::shared_traits::find_by_rowid::FindByRowID;

use super::Release;

impl FindByRowID for Release {
    async fn find_by_rowid(
        conn: &mut sqlx::SqliteConnection,
        id: i64,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(
            sqlx::query_as!(Self, "SELECT * FROM `releases` WHERE `id` = $1", id)
                .fetch_optional(conn)
                .await
                .context(SqlxSnafu)?,
        )
    }
}

impl FindByMBID for Release {
    async fn find_by_mbid(
        conn: &mut sqlx::SqliteConnection,
        id: &str,
    ) -> Result<Option<Self>, crate::Error> {
        Ok(<Self as MBIDRedirection>::find_by_mbid(conn, id).await?)
    }
}

impl Release {
    /// Search for releases by its linked url
    pub async fn find_by_url(
        conn: &mut sqlx::SqliteConnection,
        url: &str,
    ) -> Result<Vec<Release>, crate::Error> {
        Ok(sqlx::query_as(
            "SELECT releases.*
                    FROM releases
                    INNER JOIN l_releases_urls ON releases.id = l_releases_urls.`entity0`
                    INNER JOIN urls ON urls.id = l_releases_urls.`entity1`
                    WHERE urls.ressource = ?",
        )
        .bind(url)
        .fetch_all(conn)
        .await?)
    }

    /// Search for releases by its linked url
    pub async fn get_or_fetch_by_url_as_task(
        client: Arc<DBClient>,
        url: &str,
    ) -> Result<Vec<Release>, crate::Error> {
        let releases = Self::find_by_url(&mut *client.get_raw_connection().await?, url).await?;

        if releases.is_empty() {
            let url_data = Url::fetch_and_save_by_ressource_as_task(client.clone(), url).await?;

            if url_data.is_some() {
                Self::find_by_url(&mut *client.get_raw_connection().await?, url).await
            } else {
                Ok(Vec::new())
            }
        } else {
            Ok(releases)
        }
    }
}
