use crate::models::musicbrainz::user::User;
use sqlx::{query_scalar, SqliteConnection};

use super::Listen;

// #[derive(Debug, Default, Builder)]
// pub struct ListenQuery {
//     pub user: String,

//     /// Sets wether to filter out mapped listens, unmapped listens, or ignore(default)
//     pub unmapped: ListenMappingFilter,

//     /// Sets whether it should fetch the user's latest listens or not.
//     pub fetch_latest_listens: bool,
// }

// impl ListenQuery {
//     // pub async fn run(&self, client: &SqliteClient) -> Result<Vec<Listen>, Error> {
//     //     if self.fetch_latest_listens {
//     //         Listen::fetch_latest_listens_of_user(client, &self.user).await?;
//     //     }

//     //     // Ok(query_as!(
//     //     //     Listen,
//     //     //     "SELECT * FROM listens WHERE listens.user = ?",
//     //     //     self.user
//     //     // )
//     //     // .fetch_all(client.as_sqlx_pool())
//     //     // .await?)

//     //     // let querr = sqlx::query_as::<Sqlite, Listen>(
//     //     //     "SELECT
//     //     //         *
//     //     //     FROM
//     //     //         listens
//     //     //     WHERE
//     //     //         (
//     //     //             SELECT
//     //     //                 COUNT(msid_mapping.recording_msid)
//     //     //             FROM
//     //     //                 msid_mapping
//     //     //             WHERE
//     //     //                 msid_mapping.recording_msid = listens.recording_msid
//     //     //                 AND msid_mapping.user = listens.user
//     //     //         ) = 0
//     //     // "
//     //     // );
//     // }
// }

#[derive(Debug, Default)]
pub enum ListenMappingFilter {
    Mapped,
    Unmapped,
    #[default]
    Any,
}

impl Listen {
    /// Return the latest listen done by the user
    pub async fn get_latest_listen_of_user(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<Option<Listen>, sqlx::Error> {
        sqlx::query_as!(
            Listen,
            "SELECT * FROM `listens` WHERE LOWER(user) = LOWER(?) ORDER BY listened_at DESC LIMIT 1",
            user
        )
        .fetch_optional(conn)
        .await
    }

    /// Return the mapped listens of the user
    pub async fn get_mapped_listen_of_user(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<Vec<Listen>, sqlx::Error> {
        sqlx::query_as!(
            Listen,
            "
        SELECT 
            listens.*
        FROM
            users
            INNER JOIN listens ON users.name = listens.user
            INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
        WHERE
            LOWER(msid_mapping.user) = users.id
            AND 
            LOWER(listens.user) = LOWER(?)",
            user
        )
        .fetch_all(conn)
        .await
    }

    /// Return the unmapped listens of the user
    pub async fn get_unmapped_listen_of_user(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<Vec<Listen>, sqlx::Error> {
        sqlx::query_as!(
            Listen,
            "
            SELECT 
                listens.*
            FROM
                listens
                LEFT JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            WHERE
                msid_mapping.recording_mbid IS NULL
                AND 
                LOWER(listens.user) = LOWER(?)",
            user
        )
        .fetch_all(conn)
        .await
    }

    /// Get the recordings that aren't in the database but got listened by the user
    pub async fn get_unfetched_recordings_of_user(
        conn: &mut SqliteConnection,
        user: &str,
    ) -> Result<Vec<String>, sqlx::Error> {
        query_scalar!(r#"
            SELECT DISTINCT
                recordings_gid_redirect."gid"
            FROM
                users
                INNER JOIN listens ON users.name = listens.user
                INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            WHERE
                recordings_gid_redirect.deleted = 0
                AND recordings_gid_redirect.new_id IS NULL
                AND msid_mapping.user = users.id
                AND LOWER(users.name) = LOWER(?)
                "#,
            user
        )
        .fetch_all(conn)
        .await
    }

    pub async fn get_recordings_of_user(
        conn: &mut sqlx::SqliteConnection,
        user: &User,
    ) -> Result<Vec<String>, sqlx::Error> {
        query_scalar!(r#"
            SELECT DISTINCT
                recordings_gid_redirect."gid"
            FROM
                users
                INNER JOIN listens ON users.name = listens.user
                INNER JOIN messybrainz_submission ON listens.recording_msid = messybrainz_submission.msid
                INNER JOIN msid_mapping ON messybrainz_submission.msid = msid_mapping.recording_msid
                INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
            WHERE
                recordings_gid_redirect.deleted = 0
                AND msid_mapping.user = users.id
                AND users.id = ?
                "#,
            user.id
        )
        .fetch_all(conn)
        .await
    }

    pub async fn get_listens_of_recording_by_user(
        conn: &mut SqliteConnection,
        user: &str,
        recording_id: i64,
    ) -> Result<Vec<Listen>, sqlx::Error> {
        sqlx::query_as(
            "
        SELECT
            listens.*
        FROM
            users
            INNER JOIN listens ON users.name = listens.user
            INNER JOIN msid_mapping ON listens.recording_msid = msid_mapping.recording_msid
            INNER JOIN recordings_gid_redirect ON msid_mapping.recording_mbid = recordings_gid_redirect.gid
        WHERE
            -- Only for this user
            LOWER(listens.user) = LOWER(?)
            -- Keep only mapped listens 
            AND msid_mapping.user = users.id
            -- Only get those 
            AND recordings_gid_redirect.new_id = ?"
        )
        .bind(user)
        .bind(recording_id)
        .fetch_all(conn)
        .await
    }

    /// Fetch the listen using its unique triplet (listened_at, msid, username)
    pub async fn get_by_unique_triplet(
        conn: &mut sqlx::SqliteConnection,
        listened_at: i64,
        msid: &str,
        username: &str,
    ) -> Result<Option<Listen>, crate::Error> {
        Ok(sqlx::query_as!(
            Listen,
            "
                    SELECT
                        *
                    FROM
                        listens
                    WHERE
                        listened_at = ?
                        AND recording_msid = ?
                        AND user = ?;",
            listened_at,
            msid,
            username
        )
        .fetch_optional(conn)
        .await?)
    }
}
