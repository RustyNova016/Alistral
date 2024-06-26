use regex::Regex;

use crate::models::data::musicbrainz::mbid::MBIDEnum;

pub fn is_string_mbid(string: &str) -> bool {
    let regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let result = regex.captures(string);

    result.is_some()
}

pub fn get_mbid_from_url(string: &str) -> Option<MBIDEnum> {
    let regex = Regex::new(r"(recording|release|album|work|release-group|url)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let caps = regex.captures(string)?;

    let mbid = caps.get(2)?.as_str().to_string();

    match caps.get(1)?.as_str() {
        "recording" => Some(MBIDEnum::Recording(mbid.into())),
        "artist" => Some(MBIDEnum::Artist(mbid.into())),
        "work" => Some(MBIDEnum::Work(mbid.into())),
        "release" => Some(MBIDEnum::Release(mbid.into())),
        "release-group" | "album" => Some(MBIDEnum::ReleaseGroup(mbid.into())),
        _ => None,
    }
}
