use regex::Regex;

use crate::models::data::musicbrainz::mbid::state_id::any::any_entity::AnyEntityMBID;
use crate::models::data::musicbrainz::mbid::state_id::state::NaiveIDState;
use crate::models::data::musicbrainz::mbid::MBID;

pub fn is_string_mbid(string: &str) -> bool {
    let regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let result = regex.captures(string);

    result.is_some()
}

pub fn get_mbid_from_url(string: &str) -> Option<MBID> {
    let regex = Regex::new(r"(recording|release|album|work|release-group|url)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let caps = regex.captures(string)?;

    let mbid = caps.get(2)?.as_str().to_string();

    match caps.get(1)?.as_str() {
        "recording" => Some(MBID::Recording(mbid.into())),
        "artist" => Some(MBID::Artist(mbid.into())),
        "work" => Some(MBID::Work(mbid.into())),
        "release" => Some(MBID::Release(mbid.into())),
        "release-group" | "album" => Some(MBID::ReleaseGroup(mbid.into())),
        _ => None,
    }
}

pub fn parse_mbid_from_url(string: &str) -> Option<AnyEntityMBID<NaiveIDState>> {
    let regex = Regex::new(r"(recording|release|album|work|release-group|url)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let caps = regex.captures(string)?;

    let mbid = caps.get(2)?.as_str().to_string();

    match caps.get(1)?.as_str() {
        "recording" => Some(AnyEntityMBID::Recording(mbid.into())),
        "artist" => Some(AnyEntityMBID::Artist(mbid.into())),
        "work" => Some(AnyEntityMBID::Work(mbid.into())),
        "release" => Some(AnyEntityMBID::Release(mbid.into())),
        "release-group" | "album" => Some(AnyEntityMBID::ReleaseGroup(mbid.into())),
        _ => None,
    }
}
