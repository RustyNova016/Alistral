use crate::Artist;
use crate::models::shared_traits::Upsertable;

impl Artist {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `artists` (`id`, `mbid`, `name`, `sort_name`, `disambiguation`, `country`, `annotation`, `full_update_date`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `name` = excluded.`name`, `sort_name` = excluded.`sort_name`, `disambiguation` = excluded.`disambiguation`, `country` = excluded.`country`, `annotation` = excluded.`annotation`, `full_update_date` = excluded.`full_update_date` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.name);
        query = query.bind(&self.sort_name);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.country);
        query = query.bind(&self.annotation);
        query = query.bind(self.full_update_date);
        Ok(query.fetch_one(conn).await?)
    }
}

impl Upsertable for Artist {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
