use crate::LabelInfo;

impl LabelInfo {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `label_infos` (`id`, `catalog_number`, `label`, `release`) VALUES (NULL, ?, ?, ?) ON CONFLICT DO UPDATE SET `catalog_number` = excluded.`catalog_number`, `label` = excluded.`label`, `release` = excluded.`release` RETURNING *;",
        );
        query = query.bind(&self.catalog_number);
        query = query.bind(&self.label);
        query = query.bind(&self.release);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for LabelInfo {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
