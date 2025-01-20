use core::fmt::Display;

use crate::models::lookup_reports::recordings::RecordingLookupReport;
use crate::models::lookup_reports::ARROW_DOWN_RED;
use crate::models::lookup_reports::ARROW_UP_GREEN;
use crate::models::lookup_reports::DASH_GREY;

pub(super) struct RankLine {
    rank: usize,
    previous: usize,
}

impl RankLine {
    pub fn from_report(report: &RecordingLookupReport) -> Self {
        Self {
            rank: report
                .all_current()
                .get_rank(report.recording())
                .unwrap_or(0),
            previous: report
                .all_previous()
                .get_rank(report.recording())
                .unwrap_or(0),
        }
    }

    fn get_arrow(&self) -> &str {
        match self.rank.cmp(&self.previous) {
            core::cmp::Ordering::Greater => &ARROW_DOWN_RED,
            core::cmp::Ordering::Less => &ARROW_UP_GREEN,
            core::cmp::Ordering::Equal => &DASH_GREY
        }
    }
}

impl Display for RankLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Rank: #{} [{} - {}]", self.rank, self.get_arrow(), self.previous)
    }
}
