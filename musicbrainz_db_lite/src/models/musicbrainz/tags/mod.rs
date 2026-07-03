use sequelles::has_rowid::HasRowID;
use sqlx::AssertSqlSafe;
use sqlx::prelude::FromRow;
#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsyncDyn;
#[cfg(feature = "pretty_format")]
use tuillez::reexports::async_trait;

use crate::models::musicbrainz::MusicbrainzEntity;
#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::shared_traits::has_tags::HasTags;

pub mod query;

// TODO: Rework to make each entity have its own struct
#[derive(PartialEq, Eq, Debug, Clone, FromRow, sequelles::Table)]
// #[sequelles(db_name = "tags")]
// #[sequelles(sqlite)]
// #[sequelles(upsert)]
// #[sequelles(primary_key(key_name = "pk", columns(id)))]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub count: Option<i64>,
    pub score: Option<i64>,
}

impl Tag {
    pub async fn upsert<T: HasTags>(
        &mut self,
        conn: &mut sqlx::SqliteConnection,
        foreign_key: i64,
    ) -> Result<(), crate::Error> {
        let returned: Tag = sqlx::query_as(AssertSqlSafe(
            format!(
                "
        INSERT INTO
            `{}_tag` (
                `name`,
                `count`,
                `score`,
                `{}`
            )
        VALUES
            (?, ?, ?, ?)
        ON CONFLICT DO
        UPDATE
        SET
            `count` = excluded.`count`,
            `score` = excluded.`score`
        RETURNING *;",
                T::TABLE_NAME,
                T::FOREIGN_FIELD_NAME
            )
            .as_str(),
        ))
        .bind(&self.name)
        .bind(self.count)
        .bind(self.score)
        .bind(foreign_key)
        .fetch_one(conn)
        .await?;

        *self = returned;

        Ok(())
    }
}

impl HasRowID for Tag {
    fn rowid(&self) -> i64 {
        self.id
    }
}

#[cfg(feature = "pretty_format")]
#[async_trait]
impl FormatWithAsyncDyn<MusicbrainzFormater> for Tag {
    type Error = crate::Error;

    async fn format_with_async(&self, _ft: &MusicbrainzFormater) -> Result<String, Self::Error> {
        Ok(self.name.to_string())
    }
}

impl MusicbrainzEntity for Tag {
    fn entity_name() -> &'static str {
        "tag"
    }
}