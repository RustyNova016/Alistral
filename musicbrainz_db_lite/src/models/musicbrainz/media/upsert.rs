use crate::Media;

impl Media {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `medias` (`id`, `track_count`, `title`, `position`, `disc_count`, `format`, `track_offset`, `release`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `track_count` = excluded.`track_count`, `title` = excluded.`title`, `position` = excluded.`position`, `disc_count` = excluded.`disc_count`, `format` = excluded.`format`, `track_offset` = excluded.`track_offset`, `release` = excluded.`release` RETURNING *;",
        );
        query = query.bind(self.track_count);
        query = query.bind(&self.title);
        query = query.bind(self.position);
        query = query.bind(self.disc_count);
        query = query.bind(&self.format);
        query = query.bind(self.track_offset);
        query = query.bind(self.release);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for Media {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
