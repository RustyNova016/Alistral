use regex::Regex;

pub fn is_string_mbid(string: &str) -> bool {
    let regex = Regex::new(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
    )
    .unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let result = regex.captures(string);

    result.is_some()
}

pub fn get_raw_mbid_from_url(string: &str) -> Option<String> {
    let regex = Regex::new(r"(area|instrument|recording|release|album|work|release-group|url|track)/([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12})").unwrap();

    // result will be a tuple containing the start and end indices for the first match in the string
    let caps = regex.captures(string)?;

    Some(caps.get(2)?.as_str().to_string())
}
