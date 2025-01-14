use thiserror::Error;

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum Error {
    #[error(transparent)]
    MusicbrainzDBLiteError(#[from] musicbrainz_db_lite::Error),
}
