use crate::ReleaseGroup;

impl ReleaseGroup {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `release_groups` (`id`, `mbid`, `title`, `primary_type_id`, `first_release_date`, `disambiguation`, `annotation`, `artist_credit`, `full_update_date`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `title` = excluded.`title`, `primary_type_id` = excluded.`primary_type_id`, `first_release_date` = excluded.`first_release_date`, `disambiguation` = excluded.`disambiguation`, `annotation` = excluded.`annotation`, `artist_credit` = excluded.`artist_credit`, `full_update_date` = excluded.`full_update_date` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.title);
        query = query.bind(&self.primary_type_id);
        query = query.bind(self.first_release_date);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.annotation);
        query = query.bind(self.artist_credit);
        query = query.bind(self.full_update_date);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for ReleaseGroup {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
