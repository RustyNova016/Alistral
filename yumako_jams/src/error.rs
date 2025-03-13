use musicbrainz_db_lite::database::pool::DBLitePoolError;
use musicbrainz_db_lite::database::raw_conn_pool::RawPoolError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "A variable was missing during the radio compilation. Please provide it \nMissing variable path: `{0}.{1}`"
    )]
    MissingVariableError(String, String),

    #[error("Variable {0} has the wrong type. Expected `{1}`, got `{2}`")]
    WrongVariableTypeError(String, String, String),

    #[error("Couldn't compile the radio due to incorect variable: {0}. \nStep id: `{1}`")]
    VariableReadError(serde_json::Error, String),

    #[error("Couldn't compile the radio due to incorect variable: {0}. Hint: {1}")]
    VariableDecodeError(String, String),

    #[error("Couldn't deserialize the radio. Please check for errors in the schema: {0}")]
    RadioReadError(serde_json::Error),

    #[error(
        "A variable path isn't properly constructed. Expected format `step_id.input_name`, found: `{0}`"
    )]
    VariablePathError(String),

    #[error("Unknown step type `{0}`. Please check for typos")]
    UnknownStepTypeError(String),

    #[error(transparent)]
    DBConnectionError(#[from] DBLitePoolError),

    #[error(transparent)]
    DBRawConnectionError(#[from] RawPoolError),

    #[error(transparent)]
    AlistralCoreError(#[from] alistral_core::Error),

    #[error(transparent)]
    MBDBliteeError(#[from] musicbrainz_db_lite::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
