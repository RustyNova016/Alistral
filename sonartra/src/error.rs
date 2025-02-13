use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Te")]
    ReadVariableError(serde_json::Error),

    #[error("A variable was missing during the radio compilation. Step id: {0}, variable name: {1}")]
    MissingVariableError(String, String),

    #[error("Variable {0} has the wrong type. Expected `{1}`, got `{2}`")]
    WrongVariableTypeError(String, String, String),

    #[error("Couldn't compile the radio due to incorect variables: {0}")]
    VariableReadError(serde_json::Error),
}
