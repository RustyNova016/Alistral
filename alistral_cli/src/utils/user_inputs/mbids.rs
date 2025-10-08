use regex::Regex;

use crate::utils::user_inputs::UserInputParser;

impl UserInputParser {
    /// Return true if the input is an uuid
    pub fn is_uuid(mbid: &str) -> bool {
        let regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();

        // result will be a tuple containing the start and end indices for the first match in the string
        let result = regex.captures(mbid);

        result.is_some()
    }

    /// Retrieve the mbid from an url
    pub fn get_mbid_from_url(string: &str) -> Option<String> {
        let regex = Regex::new(r"(area|instrument|recording|release|album|work|release-group|url|track)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

        // result will be a tuple containing the start and end indices for the first match in the string
        let caps = regex.captures(string)?;

        Some(caps.get(2)?.as_str().to_string())
    }

    /// Get the MBID from an input string
    pub fn read_mbid_from_input(input: &str) -> Option<String> {
        if UserInputParser::is_uuid(input) {
            return Some(input.to_string());
        }

        Self::get_mbid_from_url(input)
    }
}
