use std::sync::Arc;

use crate::DBClient;

pub mod artist;
pub mod artist_credit;
pub mod genre;
pub mod isrc;
pub mod label;
pub mod main_entities;
pub mod recording;
pub mod relations;
pub mod release;
pub mod release_group;
pub mod tags;
pub mod track;
pub mod url;
pub mod user;
pub mod work;

pub struct MusicbrainzFormater {
    pub client: Arc<DBClient>,

    /// Use Listenbrainz URLs instead of Musicbrainz
    pub listenbrainz_link: bool,

    /// Add the artist credits of the enitity as well.
    pub artist_credits: bool,
}
