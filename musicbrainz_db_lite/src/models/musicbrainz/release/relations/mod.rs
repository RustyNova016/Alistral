use std::sync::Arc;

use futures::SinkExt as _;
use futures::channel::mpsc::Sender;
use sqlx::SqliteConnection;

use crate::Artist;
use crate::ArtistCredit;
use crate::DBClient;
use crate::DBRelation;
use crate::FetchAsComplete;
use crate::Label;
use crate::Media;
use crate::Track;
use crate::models::musicbrainz::main_entities::MainEntity;
use crate::models::shared_traits::db_relation::ArtistCreditDBRel;
use crate::models::shared_traits::db_relation::ArtistFromCreditsRelation;

use super::Release;

pub mod label_infos;
pub mod labels;
pub mod recording;
pub mod release_group;
pub mod urls;

impl Release {
    /// Get the releases of the recording, and fetch them if necessary.
    pub async fn get_medias_or_fetch(
        &self,
        conn: &mut SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<Media>, crate::Error> {
        // First, make sure all the data of the entity is in the database
        let id = self.get_or_fetch_as_complete(conn, client).await?.id;

        // Next, get all the children
        Ok(sqlx::query_as(
            r#"SELECT
                    medias.*
                FROM
                    releases
                    INNER JOIN medias ON releases.id = medias.release
                WHERE
                    releases.id = ?"#,
        )
        .bind(id)
        .fetch_all(conn)
        .await?)
    }

    pub async fn get_crawler(
        &self,
        client: Arc<DBClient>,
        mut sender: Sender<Arc<MainEntity>>,
    ) -> Result<(), crate::Error> {
        Self::fetch_mbid_as_complete_as_task(client.clone(), &self.mbid).await?;

        let recordings = self
            .get_recordings_or_fetch(&mut *client.get_raw_connection().await?, &client)
            .await?;
        for recording in recordings {
            sender
                .send(Arc::new(MainEntity::Recording(recording)))
                .await?;
        }

        let tracks = self
            .get_related_entity::<TrackReleaseDBRel>(&mut *client.get_raw_connection().await?)
            .await?;
        for track in tracks {
            sender.send(Arc::new(MainEntity::Track(track))).await?;
        }

        Ok(())
    }
}

impl DBRelation<ArtistCreditDBRel> for Release {
    type ReturnedType = ArtistCredit;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON releases.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit"
    }
}

impl DBRelation<ArtistFromCreditsRelation> for Release {
    type ReturnedType = Artist;

    fn get_join_statement() -> &'static str {
        "INNER JOIN artist_credits ON releases.artist_credit = artist_credits.id
        INNER JOIN artist_credits_item ON artist_credits.id = artist_credits_item.artist_credit
        INNER JOIN artists_gid_redirect ON artist_credits_item.artist_gid = artists_gid_redirect.gid
        INNER JOIN artists ON artists_gid_redirect.new_id = artists.id"
    }
}

/// [`crate::Track`] (N:1) -> [`crate::Release`]
pub struct TrackReleaseDBRel;

impl DBRelation<TrackReleaseDBRel> for Release {
    type ReturnedType = Track;

    fn get_join_statement() -> &'static str {
        "INNER JOIN tracks ON tracks.media = medias.id
        INNER JOIN medias ON medias.`release` = releases.id"
    }
}

/// [`crate::Release`] (N:N) -> [`crate::Label`]
pub struct ReleaseLabelDBRel;

impl DBRelation<ReleaseLabelDBRel> for Release {
    type ReturnedType = Label;

    fn get_join_statement() -> &'static str {
        "INNER JOIN label_infos ON releases.id = label_infos.release
        INNER JOIN labels ON label_infos.label = labels_gid_redirect.gid
        INNER JOIN labels_gid_redirect ON labels_gid_redirect.new_id = labels.id"
    }
}
