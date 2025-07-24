use crate::repository::error::RepositoryError;
use crate::repository::repository_file::RepositoryFile;

pub mod error;
pub mod radio_declaration;
pub mod repository_file;

pub struct RadioRepository {
    remote_location: String,
}

impl RadioRepository {
    pub async fn fetch_repo_file(&self) -> Result<RepositoryFile, RepositoryError> {
        let body = reqwest::get(&self.remote_location).await?.text().await?;

        if let Ok(file) = serde_json::from_str::<RepositoryFile>(&body) {
            return Ok(file);
        }

        if let Ok(file) = toml::from_str::<RepositoryFile>(&body) {
            return Ok(file);
        }

        Err(RepositoryError::RepoFileRead)
    }
}
