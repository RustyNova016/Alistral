use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Tried to unwrap a `None` value")]
    UnwrapNone,
}
