use crate::Work;

impl Work {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `Works` (`id`, `mbid`, `title`, `work_type`, `disambiguation`, `annotation`, `full_update_date`) VALUES (NULL, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `title` = excluded.`title`, `work_type` = excluded.`work_type`, `disambiguation` = excluded.`disambiguation`, `annotation` = excluded.`annotation`, `full_update_date` = excluded.`full_update_date` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.title);
        query = query.bind(&self.work_type);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.annotation);
        query = query.bind(self.full_update_date);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for Work {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
