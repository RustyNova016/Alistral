use std::collections::HashMap;

/// Map a listen MSID to a recording MBID in listenbrainz
pub async fn map_msid_to_mbid(msid: &str, mbid: &str, token: &str) -> Result<(), crate::Error> {
    let client = reqwest::Client::new();

    let mut body_json = HashMap::new();
    body_json.insert("recording_msid", msid.to_string());
    body_json.insert("recording_mbid", mbid.to_string());

    client
        .post("https://api.listenbrainz.org/1/metadata/submit_manual_mapping/")
        .header("Authorization", format!("Token {}", token.to_owned()))
        .json(&body_json)
        .send()
        .await?;

    Ok(())
}
