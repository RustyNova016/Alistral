use serde::Deserialize;

use crate::repository::error::RepositoryError;


#[derive(Debug, Deserialize)]
pub struct RepositoryFile {
    name: String,
    radios: Vec<String>,
}

impl RepositoryFile {
    pub fn download_files() -> Result<(), RepositoryError> {

    }

        fn download_file(file_url: &str) -> Result<(), RepositoryError> {
        
    }
}