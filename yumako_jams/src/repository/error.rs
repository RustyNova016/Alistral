use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error("Couldn't read the repository file")]
    RepoFileRead
}
