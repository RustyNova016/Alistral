use chrono::Duration;
use rust_decimal::Decimal;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListensOld;
use crate::models::config::Config;

/// Sort listens based on the rate of listens of a recording
pub fn listen_rate_sorter(
    mut recordings: Vec<RecordingWithListensOld>,
) -> Vec<RecordingWithListensOld> {
    let conf = Config::load_or_panic();

    recordings.sort_by_cached_key(|r| {
        let score = r
            .get_listen_rate(Duration::days(365))
            .unwrap_or_else(|| Decimal::from(2147483646));
        score
            * conf
                .read_or_panic()
                .bumps
                .get_multiplier(&r.recording().mbid)
    });

    recordings
}
