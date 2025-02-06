use thiserror::Error;

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum TimeError {
    #[error(transparent)]
    ParseError(humantime::DurationError),

    #[error(transparent)]
    ConvertError(chrono::OutOfRangeError)
}

