pub mod fetch_url;
use std::sync::Arc;

use futures::FutureExt;
use futures::future::BoxFuture;
use musicbrainz_rs::Fetch;

use sqlx::Acquire;

use crate::CompletenessFlag;
use crate::DBClient;
use crate::MBIDRedirection;
use crate::MBUrl;
use crate::models::musicbrainz::url::Url;
use crate::models::shared_traits::fetch_and_save::FetchAndSave;
use crate::models::shared_traits::fetch_mbid::FetchMBID;
use crate::models::shared_traits::save_from::SaveFrom;

impl FetchAndSave<MBUrl> for Url {
    async fn set_redirection(
        conn: &mut sqlx::SqliteConnection,
        mbid: &str,
        id: i64,
    ) -> Result<(), sqlx::Error> {
        Url::link_mbid(conn, mbid, id).await
    }
}

impl CompletenessFlag for Url {
    async fn set_full_update(
        &mut self,
        _conn: &mut sqlx::SqliteConnection,
    ) -> Result<(), sqlx::Error> {
        Ok(())
    }

    fn is_complete(&self) -> bool {
        true
    }
}

impl From<MBUrl> for Url {
    fn from(value: MBUrl) -> Self {
        Self {
            id: 0,
            mbid: value.id,
            ressource: value.resource,
        }
    }
}

impl SaveFrom<MBUrl> for Url {
    async fn save_from(
        conn: &mut sqlx::SqliteConnection,
        value: MBUrl,
    ) -> Result<Self, crate::Error> {
        Url::save_api_response(conn, value).await
    }
}

impl FetchMBID<MBUrl> for Url {
    async fn fetch_from_mbid(
        client: &crate::DBClient,
        mbid: &str,
    ) -> Result<MBUrl, musicbrainz_rs::GetRequestError> {
        MBUrl::fetch()
            .id(mbid)
            .with_area_relations()
            .with_artist_relations()
            .with_event_relations()
            .with_genre_relations()
            .with_instrument_relations()
            .with_instrument_relations()
            .with_label_relations()
            .with_place_relations()
            .with_recording_relations()
            .with_release_group_relations()
            .with_release_group_relations()
            .with_release_relations()
            .with_series_relations()
            .with_url_relations()
            .with_work_relations()
            .execute_with_client(&client.musicbrainz_client)
            .await
    }
}

impl Url {
    pub fn save_api_response(
        conn: &mut sqlx::SqliteConnection,
        value: MBUrl,
    ) -> BoxFuture<'_, Result<Self, crate::Error>> {
        Self::save_api_response_inner(conn, value).boxed()
    }

    pub async fn save_api_response_as_task(
        client: Arc<DBClient>,
        value: MBUrl,
    ) -> Result<Self, crate::Error> {
        tokio::spawn(async move {
            Self::save_api_response_inner(&mut *client.get_raw_connection().await?, value).await
        })
        .await
        .unwrap()
    }

    async fn save_api_response_inner(
        conn: &mut sqlx::SqliteConnection,
        value: MBUrl,
    ) -> Result<Self, crate::Error> {
        let mut conn = conn.begin().await?;

        let output = Url::from(value.clone()).upsert(&mut conn).await?;

        if let Some(relations) = value.relations {
            // Remove all the old relations
            output.delete_all_relations(&mut conn).await?;

            for rel in relations {
                match output.save_relation(&mut conn, rel).await {
                    Ok(_) => {}
                    Err(crate::Error::RelationNotImplemented) => {}
                    Err(err) => {
                        Err(err)?;
                    }
                }
            }
        }

        conn.commit().await?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {

    use core::future::ready;

    use futures::StreamExt as _;
    use futures::stream;

    use crate::models::musicbrainz::recording::Recording;
    use crate::models::musicbrainz::url::Url;
    use crate::models::shared_traits::fetch_and_save::FetchAndSave;
    use crate::tests::fixtures::default_client::test_mb_client;

    #[tokio::test]
    #[serial_test::serial]
    async fn should_insert_url() {
        let client = test_mb_client();

        // Test values. Feel free to add edge cases here
        let test_values = vec!["7b62f7c5-2100-48de-8973-2a243482b0bb"];

        for mbid in test_values {
            let value = Url::fetch_and_save_with_pool(&client, mbid).await.unwrap();

            assert!(value.is_some());
        }
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn should_get_url_relations() {
        let client = test_mb_client();

        let recording_with_url =
            Recording::fetch_and_save_with_pool(&client, "490ec335-1972-4b06-86e6-1dd8d5484cf7")
                .await
                .unwrap()
                .unwrap();
        let relations = recording_with_url
            .get_url_relations(&mut client.get_raw_connection().await.unwrap())
            .await
            .unwrap();

        let has_relation = stream::iter(relations)
            .map(async |rel| {
                rel.get_entity_1_as_right(&mut client.get_raw_connection().await.unwrap())
                    .await
            })
            .buffer_unordered(8)
            .map(|url| url.unwrap())
            .any(|url| ready(url.mbid == "0726e922-1237-4c46-a320-48212c4e95a2"))
            .await;

        assert!(has_relation)
    }
}
