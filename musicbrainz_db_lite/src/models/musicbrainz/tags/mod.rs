use sequelles::has_rowid::HasRowID;
use sqlx::prelude::FromRow;
#[cfg(feature = "pretty_format")]
use tuillez::formatter::FormatWithAsyncDyn;
#[cfg(feature = "pretty_format")]
use tuillez::reexports::async_trait;

#[cfg(feature = "pretty_format")]
use crate::models::musicbrainz::MusicbrainzFormater;
use crate::models::shared_traits::has_tags::HasTags;

pub mod query;

#[derive(PartialEq, Eq, Debug, Clone, FromRow)]
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
        let returned: Tag = sqlx::query_as(&format!(
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
