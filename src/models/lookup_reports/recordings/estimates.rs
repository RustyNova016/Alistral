use std::fmt::Write;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use humantime::format_duration;

use crate::models::lookup_reports::recordings::RecordingLookupReport;
use crate::utils::extensions::chrono_ext::DurationExt as _;

impl RecordingLookupReport {
    pub(super) fn fmt_average_dur_between_listens(&self) -> String {
        let mut out = "Average duration between listens: ".to_string();

        write!(
            out,
            "{}",
            format_duration(
                self.recording_current()
                    .expect("There should be at least one recording")
                    .average_duration_between_listens()
                    .floor_to_minute()
                    .to_std()
                    .unwrap()
            )
        ).unwrap();

        if self.all_time {
            write!(
            out,
            " [{} - {}]",
            self.get_arrow(now, then, lower_better)
            format_duration(
                self.recording_current()
                    .expect("There should be at least one recording")
                    .average_duration_between_listens()
                    .floor_to_minute()
                    .to_std()
                    .unwrap()
            )
        ).unwrap();
        }

        out
    }
}
