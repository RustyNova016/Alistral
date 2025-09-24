use sqlx::SqliteConnection;

use crate::LabelInfo;
use crate::models::musicbrainz::release::Release;

impl Release {
    pub async fn get_label_infos_or_fetch(
        &self,
        conn: &mut SqliteConnection,
        client: &crate::DBClient,
    ) -> Result<Vec<LabelInfo>, crate::Error> {
        // First, make sure all the data of the entity is in the database
        let id = self.get_or_fetch_as_complete(conn, client).await?.id;

        // Next, get all the children
        Ok(sqlx::query_as!(
            LabelInfo,
            "SELECT
                    label_infos.*
                FROM 
                    releases
                    INNER JOIN label_infos ON releases.id = label_infos.release
                WHERE
                    releases.id = ?",
            id
        )
        .fetch_all(conn)
        .await?)
    }
}
