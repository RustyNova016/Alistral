use std::fs::File;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::json::radio::Radio;
use crate::repository::error::RepositoryError;

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryFile {
    name: String,
    radios: Vec<String>,
}

impl RepositoryFile {
   // pub fn download_files() -> Result<(), RepositoryError> {}



    pub fn get_repo_folder(&self, repo_root: &Path) -> PathBuf {
        // Check if our name has a `/` or `\`. We don't want subfolders to get created. We also check if it's an absolute path
        if self.name.contains("\\")
            || self.name.contains("/")
            || PathBuf::from(&self.name).is_absolute()
        {
            panic!("Absolute path in repository name.")
        }

        repo_root.join(&self.name)
    }
}
