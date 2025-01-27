pub mod client;
pub mod database;
pub mod error;
pub mod models;
pub mod utils;

pub use crate::client::InterzicClient;
pub use crate::error::Error;

pub mod reexports {
    pub use musicbrainz_rs::client::MUSICBRAINZ_CLIENT;
}
