use std::sync::Arc;

use musicbrainz_db_lite::DBClient;

pub struct SymphonyzeClient {
    pub mb_database: Arc<DBClient>,
}

impl SymphonyzeClient {
    #[cfg(test)]
    pub async fn get_testing_client(name: &str) -> Self {
        let db_client = DBClient::get_default_named(name).expect("Couldn't create database client");

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
