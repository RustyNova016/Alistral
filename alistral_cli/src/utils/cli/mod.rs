use core::fmt;
use std::io;

use clap::CommandFactory as _;

use super::regex::is_string_mbid;
use crate::models::cli::Cli;
use crate::utils::regex::get_raw_mbid_from_url;

pub mod formating;
pub mod parsing;

/// Block the current trhead until the user press enter
pub fn await_next() {
    let buf = &mut String::new();
    let _ = io::stdin().read_line(buf);
}

pub fn read_mbid_from_input(input: &str) -> Option<String> {
    if is_string_mbid(input) {
        return Some(input.to_string());
    }

    get_raw_mbid_from_url(input)
}

pub fn clap_error(msg: impl fmt::Display, error: clap::error::ErrorKind) -> ! {
    Cli::command().error(error, msg).exit()
}
