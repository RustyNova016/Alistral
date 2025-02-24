pub mod label_infos;
use sqlx::SqliteConnection;

use super::Media;
use super::Release;

pub mod labels;
pub mod recording;
pub mod release_group;

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
}
