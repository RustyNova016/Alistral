use std::collections::HashMap;

use alistral_core::cli::progress_bar::ProgressBarCli;

use crate::models::data::listenbrainz::popularity::PopularityRecordingResponseItem;

pub async fn get_global_listen_counts(
    recordings: &[String],
) -> Result<Vec<PopularityRecordingResponseItem>, crate::Error> {
    let mut results = Vec::new();
    let client = reqwest::Client::new();
    let progress = ProgressBarCli::new(
        (recordings.len() / 999) as u64,
        Some("Getting global statistics"),
    );

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
        progress.inc(1);
    }

    Ok(results)
}
