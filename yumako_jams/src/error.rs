use core::fmt::Display;

use alistral_core::datastructures::entity_with_listens::recording::collection::sort::RecordingStatsError;
use musicbrainz_db_lite::GetConnectionError;
use musicbrainz_db_lite::models::listenbrainz::listen::selects::error::ListenFetchGetError;
use thiserror::Error;

use crate::modules::error::StreamModuleError;


#[derive(Error, Debug)]
pub enum Error {
    #[error(
        "A variable was missing during the radio compilation. Please provide it \nMissing variable path: `{0}`"
    )]
    MissingVariableError(String),

    #[error("Variable {0} has the wrong type. Expected `{1}`, got `{2}`")]
    WrongVariableTypeError(String, String, String),

    #[error(transparent)]
    VariableTypeError(VariableTypeError),

    #[error("Couldn't compile the radio due to incorrect variable: {0}. \nStep id: `{1}`")]
    VariableReadError(serde_json::Error, String),

    #[error("Couldn't compile the radio due to incorrect variable: {0}. Hint: {1}")]
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
    AlistralCoreError(#[from] alistral_core::Error),

    #[error(transparent)]
    MBDBliteeError(#[from] musicbrainz_db_lite::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(
        "Couldn't parse the radio file. Make sure you have a proper schema.\nJSON Error: {0}\nTOML Error: {1}"
    )]
    RadioFileTypeError(serde_json::Error, toml::de::Error),

    #[error(transparent)]
    ListenFetchGetError(#[from] ListenFetchGetError),

    #[error(transparent)]
    RecordingStatsError(#[from] RecordingStatsError),

    #[error(transparent)]
    GetConnectionError(#[from] GetConnectionError),

    #[error(transparent)]
    StreamModuleError(#[from] StreamModuleError),
}

impl Error {
    pub fn new_variable_type_error<T: core::error::Error + Send + 'static>(
        variable_name: String,
        variable_type: String,
        data: String,
        error: T,
    ) -> Self {
        Self::VariableTypeError(VariableTypeError {
            variable_name,
            variable_type,
            data,
            serde_error: Box::new(error),
        })
    }

    pub fn new_missing_variable_error(layer: &str, variable: &str) -> Self {
        Self::MissingVariableError(format!("{layer}.{variable}"))
    }
}

#[derive(Debug)]
pub struct VariableTypeError {
    variable_name: String,
    variable_type: String,
    data: String,
    serde_error: Box<dyn core::error::Error + Send>,
}

impl Display for VariableTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Found invalid variable type. Variable `{}` is declared as type `{}`, but got `{}`, which is not deserialable to this type.",
            self.variable_name, self.variable_type, self.data
        )?;
        writeln!(f, "The deserializer returned: {}", self.serde_error)?;

        Ok(())
    }
}

impl core::error::Error for VariableTypeError {}
