use core::ops::AddAssign;
use std::collections::HashMap;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Duration;
use chrono::DurationRound;
use chrono::Timelike;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::interface::comp_arrow::ComparisonArrow;
use crate::models::hour_min::HourMinute;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub(super) async fn best_listen_hour(&self) -> String {
        let current = self.current.recording_stats().await.unwrap();
        let previous = self.previous.recording_stats().await.unwrap();

        let current_top = calculate_best_listen_hours_for_recording_coll(current);
        let current_top = current_top.iter().max_by_key(|(_, count)| *count).unwrap();

        let previous_top = calculate_best_listen_hours_for_recording_coll(previous);
        let previous_top = previous_top.iter().max_by_key(|(_, count)| *count).unwrap();

        format!(
            "Listened the most tracks at {} ({} listens). Last year was {} ({} {} listens)",
            current_top.0.alistral_green(),
            current_top.1.alistral_green(),
            previous_top.0.alistral_green(),
            ComparisonArrow::greater_is_better(current_top.1, previous_top.1),
            previous_top.1.alistral_green()
        )
    }
}

fn calculate_best_listen_hours_for_recording_coll(
    recs: &RecordingWithListensCollection,
) -> HashMap<HourMinute, u64> {
    let time_iter = recs.iter().flat_map(calculate_listen_hours_for_recording);
    let mut counter: HashMap<HourMinute, u64> = HashMap::new();

    for time in time_iter {
        counter.entry(time).or_default().add_assign(1);
    }

    counter
}

fn calculate_listen_hours_for_recording(rec: &RecordingWithListens) -> Vec<HourMinute> {
    let duration = rec.entity().length_as_duration().unwrap_or_default();
    let mut out = Vec::with_capacity(1);

    for listen in rec.iter_listens() {
        out.extend(calculate_listen_hours_for_listen(listen, duration));
    }

    out
}

/// Return the (Hour minute) where that listen was listened at
fn calculate_listen_hours_for_listen(
    listen: &Listen,
    recording_duration: Duration,
) -> Vec<HourMinute> {
    let start = listen.listened_at_as_datetime();
    let end = (start + recording_duration)
        .duration_round_up(Duration::minutes(1))
        .unwrap();
    let mut current_time = start;
    let mut out = Vec::with_capacity(1);

    while current_time <= end {
        let time = HourMinute {
            hours: current_time.hour() as u8,
            minutes: current_time.minute() as u8,
        };

        out.push(time);

        current_time += Duration::minutes(1)
    }

    out
}
