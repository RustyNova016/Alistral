use std::io;

use crate::utils::regex::get_raw_mbid_from_url;
use crate::utils::user_inputs::UserInputParser;

pub mod formating;

/// Block the current trhead until the user press enter
pub fn await_next() {
    let buf = &mut String::new();
    let _ = io::stdin().read_line(buf);
}

pub fn read_mbid_from_input(input: &str) -> Option<String> {
    if UserInputParser::is_uuid(input) {
        return Some(input.to_string());
    }

    get_raw_mbid_from_url(input)
}
