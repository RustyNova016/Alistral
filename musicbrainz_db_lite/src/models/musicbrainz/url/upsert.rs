use crate::Url;

impl Url {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `urls` (`id`, `mbid`, `ressource`) VALUES (NULL, ?, ?) ON CONFLICT DO UPDATE SET `ressource` = excluded.`ressource` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.ressource);
        Ok(query.fetch_one(conn).await?)
    }
}

impl crate::models::shared_traits::Upsertable for Url {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
