pub mod client;
pub mod errors;
pub mod listenbrainz;
pub mod musicbrainz;
pub mod shared_traits;

use core::ops::Deref;

pub struct RowID(pub i64);

impl Deref for RowID {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
