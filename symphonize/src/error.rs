#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Tried to unwrap a `None` value")]
    UnwrapNone,
}

