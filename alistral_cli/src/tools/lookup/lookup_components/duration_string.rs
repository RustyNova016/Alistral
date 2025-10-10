use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

pub struct LookupDurationString(pub Option<Duration>);

impl Display for LookupDurationString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(dur) = &self.0 else {
            write!(f, "<unknown>")?;
            return Ok(());
        };

        let dur_mins = dur.deci_minutes().trunc_with_scale(2);
        let dur_hours = dur.format_hh_mm();

        write!(f, "{dur_mins} minutes ({dur_hours} hours)")?;

        Ok(())
    }
}
