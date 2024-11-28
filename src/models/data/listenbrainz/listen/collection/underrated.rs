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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct PopularityRecordingResponseItem {
    //TODO: Replace with listenbrainz_rs's
    pub recording_mbid: String,
    pub total_listen_count: Option<u64>,
    pub total_user_count: Option<u64>,
}
