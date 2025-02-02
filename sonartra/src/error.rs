use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Te")]
    ReadVariableError(serde_json::Error),
}
