use std::sync::Arc;

use musicbrainz_db_lite::DBClient;

pub struct SymphonyzeClient {
    pub mb_database: Arc<DBClient>,
}

impl SymphonyzeClient {
    #[cfg(test)]
    pub async fn get_testing_client(name: &str) -> Self {
        let db_client = DBClient::builder()
            .in_memory_with_name(name.to_string())
            .set_default_lb_client()
            .set_default_mb_client()
            .connect_and_migrate(8)
            .await
            .unwrap()
            .build();

        Self {
            mb_database: Arc::new(db_client),
        }
    }

    #[cfg(test)]
    pub async fn load_test_data(&self, file: &str) {
        use core::str::FromStr as _;
        use std::path::PathBuf;

        let mut path = PathBuf::from_str("./testing_data").unwrap();
        path.push(file);

        self.mb_database.load_datafile(&path).await.unwrap();
    }
}
