use musicbrainz_db_lite::database::pool::DBLitePoolError;
use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "A variable was missing during the radio compilation. Step id: {0}, variable name: {1}"
    )]
    MissingVariableError(String, String),

    #[error("Variable {0} has the wrong type. Expected `{1}`, got `{2}`")]
    WrongVariableTypeError(String, String, String),

    #[error("Couldn't compile the radio due to incorect variable: {0}")]
    VariableReadError(serde_json::Error),

    #[error("Couldn't compile the radio due to incorect variable: {0}. Hint: {1}")]
    VariableDecodeError(String, String),

    #[error(transparent)]
    DBConnectionError(#[from] DBLitePoolError),

    #[error(transparent)]
    DBRawConnectionError(#[from] RawPoolError),

    #[error(transparent)]
    AlistralCoreError(#[from] alistral_core::Error),
}
