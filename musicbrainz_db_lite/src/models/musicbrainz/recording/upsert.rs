use crate::Recording;
use crate::models::shared_traits::Upsertable;

impl Recording {
    pub async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        let mut query = sqlx::query_as(
            "INSERT INTO `recordings` (`id`, `mbid`, `title`, `video`, `length`, `disambiguation`, `annotation`, `first_release_date`, `full_update_date`, `artist_credit`) VALUES (NULL, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO UPDATE SET `title` = excluded.`title`, `video` = excluded.`video`, `length` = excluded.`length`, `disambiguation` = excluded.`disambiguation`, `annotation` = excluded.`annotation`, `first_release_date` = excluded.`first_release_date`, `full_update_date` = excluded.`full_update_date`, `artist_credit` = excluded.`artist_credit` RETURNING *;",
        );
        query = query.bind(&self.mbid);
        query = query.bind(&self.title);
        query = query.bind(self.video);
        query = query.bind(self.length);
        query = query.bind(&self.disambiguation);
        query = query.bind(&self.annotation);
        query = query.bind(self.first_release_date);
        query = query.bind(self.full_update_date);
        query = query.bind(self.artist_credit);

        Ok(query.fetch_one(conn).await?)
    }
}

impl Upsertable for Recording {
    async fn upsert(&self, conn: &mut sqlx::SqliteConnection) -> Result<Self, crate::Error> {
        self.upsert(conn).await
    }
}
