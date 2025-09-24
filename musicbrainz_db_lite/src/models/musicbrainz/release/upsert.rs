use crate::Release;
use crate::models::shared_traits::Upsertable;

impl Release {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `releases` (`id`, `mbid`, `title`, `date`, `country`, `quality`, `status`, `barcode`, `disambiguation`, `packaging`, `annotation`, `full_update_date`, `artist_credit`, `release_group`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `title` = excluded.`title`, `date` = excluded.`date`, `country` = excluded.`country`, `quality` = excluded.`quality`, `status` = excluded.`status`, `barcode` = excluded.`barcode`, `disambiguation` = excluded.`disambiguation`, `packaging` = excluded.`packaging`, `annotation` = excluded.`annotation`, `full_update_date` = excluded.`full_update_date`, `artist_credit` = excluded.`artist_credit`, `release_group` = excluded.`release_group` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.title);
        query = query.bind(self.date);
        query = query.bind(&self.country);
        query = query.bind(&self.quality);
        query = query.bind(&self.status);
        query = query.bind(&self.barcode);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.packaging);
        query = query.bind(&self.annotation);
        query = query.bind(self.full_update_date);
        query = query.bind(self.artist_credit);
        query = query.bind(self.release_group);
        Ok(query.fetch_one(conn).await?)
    }
}

impl Upsertable for Release {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
