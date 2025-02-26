#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Tried to unwrap a `None` value")]
    DatabaseError(#[from] musicbrainz_db_lite::Error),
}
