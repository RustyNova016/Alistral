use std::collections::HashMap;

use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::models::data::listenbrainz::popularity::PopularityRecordingResponseItem;

#[instrument(fields(indicatif.pb_show = tracing::field::Empty))]
pub async fn get_global_listen_counts(
    recordings: &[String],
) -> Result<Vec<PopularityRecordingResponseItem>, crate::Error> {
    let mut results = Vec::new();
    let client = reqwest::Client::new();
    pg_counted!(recordings.len() / 999, "Getting global statistics");

    for chunk in recordings.chunks(999) {
        let mut req_body = HashMap::new();
        req_body.insert("recording_mbids", chunk);

        let res: Vec<PopularityRecordingResponseItem> = client
            .post("https://api.listenbrainz.org/1/popularity/recording")
            .json(&req_body)
            .send()
            .await?
            .json()
            .await?;

        results.extend(res);
        pg_inc!();
    }

    Ok(results)
}
