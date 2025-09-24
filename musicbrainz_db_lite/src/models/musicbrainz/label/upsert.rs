use crate::Label;

impl Label {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `labels` (`id`, `mbid`, `name`, `label_type`, `sort_name`, `disambiguation`, `country`, `label_code`, `annotation`, `full_update_date`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `name` = excluded.`name`, `label_type` = excluded.`label_type`, `sort_name` = excluded.`sort_name`, `disambiguation` = excluded.`disambiguation`, `country` = excluded.`country`, `label_code` = excluded.`label_code`, `annotation` = excluded.`annotation`, `full_update_date` = excluded.`full_update_date` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.name);
        query = query.bind(&self.label_type);
        query = query.bind(&self.sort_name);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.country);
        query = query.bind(self.label_code);
        query = query.bind(&self.annotation);
        query = query.bind(self.full_update_date);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for Label {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
