use std::fs::write;
use std::path::Path;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::json::radio::Radio;
use crate::repository::error::RepositoryError;

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct RadioDeclaration {
    name: String,
    path: String,
}

impl RadioDeclaration {
    async fn download_file(
        &self,
        repo_url_root: &str,
        repo_folder: &Path,
    ) -> Result<(), RepositoryError> {
        let content = reqwest::get(format!("{}{}", repo_url_root, self.path))
            .await?
            .text()
            .await?;

        // Check if the radio is correct before saving. We don't want to download random files off the internet
        Radio::from_file_content(&content).map_err(RepositoryError::RadioFileReadError)?;

        // Check if our name is an absolute path. This prevent overwriting the root path of the repo and put files everywhere
        if PathBuf::from(&self.name).is_absolute() {
            panic!("Absolute path in radio path") //TODO: Proper error handling
        }

        write(repo_folder.join(&self.name), content);

        Ok(())
    }
}
