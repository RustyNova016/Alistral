use std::fmt::Write;

use alistral_core::cli::colors::AlistralColors;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::models::cli_components::formaters::mh_duration_formater::MHDurationFormater;

pub(super) fn discoveries_no_previous(
    discovered: &[&RecordingWithListens],
    current_recordings: &RecordingWithListensCollection,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
) -> String {
    let discovered_track_count = discovered.len();
    let current_track_count = current_recordings.len();

    let discovered_tracks_percent = Decimal::new(discovered_track_count as i64, 0)
        / Decimal::new(current_track_count as i64, 0);

    let mut out = format!(
        "You discovered {} new tracks ({}% of the total of tracks listened)\n",
        discovered_track_count.alistral_green(),
        discovered_tracks_percent
            .checked_mul(dec!(100))
            .unwrap()
            .trunc_with_scale(2)
            .alistral_green()
    );

    // ----

    let discovered_listens_count: usize = discovered.iter().map(|r| r.listen_count()).sum();
    let current_listens_count = current_recordings.listen_count();

    let listen_percent = Decimal::new(discovered_listens_count as i64, 0)
        / Decimal::new(current_listens_count as i64, 0);

    writeln!(
        out,
        "Which makes up a total of {} listens ({}% of the total listens)",
        discovered_listens_count.alistral_green(),
        listen_percent
            .checked_mul(dec!(100))
            .unwrap()
            .trunc_with_scale(2)
            .alistral_green()
    );

    // ----

    let discovered_listens_duration: TimeDelta = discovered
        .iter()
        .map(|r| r.get_time_listened().unwrap_or_default())
        .sum();
    let current_listens_duration = current_recordings.get_time_listened().unwrap_or_default();

    let listen_percent = Decimal::new(discovered_listens_duration.num_seconds(), 0)
        / Decimal::new(current_listens_duration.num_seconds(), 0);

    let period_percent = Decimal::new(discovered_listens_duration.num_seconds(), 0)
        / Decimal::new((end_time - start_time).num_seconds(), 0);

    writeln!(
        out,
        "Corresponding to {} ({}% of the total listened time, {}% of the whole period)",
        MHDurationFormater(Some(discovered_listens_duration)).alistral_green(),
        listen_percent
            .checked_mul(dec!(100))
            .unwrap()
            .trunc_with_scale(2)
            .alistral_green(),
        period_percent
            .checked_mul(dec!(100))
            .unwrap()
            .trunc_with_scale(2)
            .alistral_green()
    );

    out
}
