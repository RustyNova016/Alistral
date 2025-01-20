use core::fmt::Display;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;

use crate::models::lookup_reports::recordings::RecordingLookupReport;
use crate::models::lookup_reports::ARROW_DOWN_RED;
use crate::models::lookup_reports::ARROW_UP_GREEN;
use crate::models::lookup_reports::DASH_GREY;

pub(super) struct ListenCountLine {
    current: usize,
    previous: usize,
}

impl ListenCountLine {
    pub fn from_report(report: &RecordingLookupReport) -> Self {
        Self {
            current: report.recording_current().listen_count(),
            previous: report.recording_current().listen_count(),
        }
    }

    fn get_arrow(&self) -> &str {
        match self.current.cmp(&self.previous) {
            core::cmp::Ordering::Greater => &ARROW_UP_GREEN,
            core::cmp::Ordering::Less => &ARROW_DOWN_RED,
            core::cmp::Ordering::Equal => &DASH_GREY,
        }
    }
}

impl Display for ListenCountLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Listen count: {} [{} - {}]",
            self.rank,
            self.get_arrow(),
            self.previous
        )
    }
}
