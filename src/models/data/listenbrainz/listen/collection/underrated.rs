use std::collections::HashMap;
use std::ops::{Div, Mul};

use color_eyre::eyre::Context;
use itertools::Itertools;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

use crate::core::display::progress_bar::ProgressBarCli;
use crate::models::cli::common::{GroupByTarget, SortSorterBy};
use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;


async fn get_listen_counts_for_recordings(
    recordings: &[RecordingMBID],
) -> color_eyre::Result<Vec<PopularityRecordingResponseItem>> {
    let mut results = Vec::new();
    let progress = ProgressBarCli::new(
        (recordings.len() / 999) as u64,
        Some("Getting global statistics"),
    );

    for chunk in recordings.chunks(999) {
        let client = reqwest::Client::new();
        let mut req_body = HashMap::new();
        req_body.insert("recording_mbids", chunk);

        let res: Vec<PopularityRecordingResponseItem> = client
            .post("https://api.listenbrainz.org/1/popularity/recording")
            .json(&req_body)
            .send()
            .await
            .context("Failed to get recording statistics")?
            .json()
            .await
            .context("Failed to parse recording statistics")?;

        results.extend(res);
        progress.inc(1);
    }

    Ok(results)
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PopularityRecordingResponseItem {
    //TODO: Replace with listenbrainz_rs's
    pub recording_mbid: String,
    pub total_listen_count: Option<u64>,
    pub total_user_count: Option<u64>,
}

fn get_recording_score_from_id(
    id: RecordingMBID,
    recording_ranks: &[(usize, String, ListenCollection)],
    recording_listen_counts: &[PopularityRecordingResponseItem],
) -> Decimal {
    let recording_rank: u64 = recording_ranks
        .iter()
        .find(|rank| rank.1 == *id)
        .map(|listen_rank| listen_rank.0 as u64)
        .unwrap_or(9999);

    let user_listen_count: u64 = recording_ranks
        .iter()
        .find(|data| data.1 == *id)
        .map(|data| data.2.len() as u64)
        .unwrap_or(0);

    let world_listen_count: u64 = recording_listen_counts
        .iter()
        .find(|data| data.recording_mbid == *id)
        .and_then(|popularity_responce| popularity_responce.total_listen_count)
        .unwrap_or(0);

    get_recording_score(recording_rank, user_listen_count, world_listen_count)
}

fn get_recording_score(rank: u64, user_listen_count: u64, world_listen_count: u64) -> Decimal {
    let rank_score = Decimal::from(1000_u64.saturating_sub(rank)).div(dec!(10));

    let listen_score = Decimal::from(user_listen_count)
        .checked_div(Decimal::from(world_listen_count))
        .unwrap_or(dec!(1))
        .min(Decimal::ONE)
        .mul(dec!(100));

    rank_score + listen_score
}
