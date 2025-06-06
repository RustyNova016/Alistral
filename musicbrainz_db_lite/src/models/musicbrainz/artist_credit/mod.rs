use std::fmt::Display;

use sqlx::SqliteConnection;
use sqlx::prelude::FromRow;

use crate::models::shared_traits::has_table::HasTable;

pub mod display;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct ArtistCredit {
    pub artist_credit: i64,
    pub position: i64,
    pub name: String,
    pub artist_gid: String,
    pub join_phrase: String,
}

impl ArtistCredit {}

impl HasTable for ArtistCredit {
    const TABLE_NAME: &str = "artist_credits_item";
    const FOREIGN_FIELD_NAME: &str = "artist_credits_item";
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct ArtistCredits(pub i64, pub Vec<ArtistCredit>);

impl ArtistCredits {
    pub async fn find_by_id(
        conn: &mut SqliteConnection,
        id: i64,
    ) -> Result<ArtistCredits, sqlx::Error> {
        let result = sqlx::query_as!(
            ArtistCredit,
            "SELECT * FROM `artist_credits_item` WHERE artist_credit = ? ORDER BY position",
            id
        )
        .fetch_all(conn)
        .await?;
        Ok(Self(id, result))
    }

    pub async fn save(
        conn: &mut SqliteConnection,
        credits: &[ArtistCredit],
    ) -> Result<Self, sqlx::Error> {
        let id = sqlx::query_scalar!("INSERT INTO `artist_credits` VALUES (NULL) RETURNING *")
            .fetch_one(&mut *conn)
            .await?;

        for row in credits {
            sqlx::query!(
                "INSERT INTO `artist_credits_item` VALUES (?, ?, ?, ?, ?)",
                id,
                row.position,
                row.name,
                row.artist_gid,
                row.join_phrase
            )
            .execute(&mut *conn)
            .await?;
        }

        Self::find_by_id(conn, id).await
    }
}

impl Display for ArtistCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.join_phrase)
    }
}
