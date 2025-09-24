use crate::Track;

impl Track {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `tracks` (`id`, `gid`, `title`, `number`, `length`, `position`, `media`, `recording`, `artist_credit`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `title` = excluded.`title`, `number` = excluded.`number`, `length` = excluded.`length`, `position` = excluded.`position`, `media` = excluded.`media`, `recording` = excluded.`recording`, `artist_credit` = excluded.`artist_credit` RETURNING *;",
        );
        query = query.bind(&self.gid);
        query = query.bind(&self.title);
        query = query.bind(&self.number);
        query = query.bind(self.length);
        query = query.bind(self.position);
        query = query.bind(self.media);
        query = query.bind(self.recording);
        query = query.bind(self.artist_credit);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for Track {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
