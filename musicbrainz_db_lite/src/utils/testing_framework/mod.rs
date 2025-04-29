//! This module contains a testing framework to easily create test databases using simple json files

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::DBClient;
use crate::utils::testing_framework::data_file::DataFile;

pub mod data_file;

impl DBClient {
    pub async fn load_datafile(&self, path: &Path) -> Result<(), crate::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let file: DataFile = serde_json::from_reader(reader)?;

        file.save_datafile(&mut *self.get_raw_connection().await?)
            .await?;

        Ok(())
    }
}
