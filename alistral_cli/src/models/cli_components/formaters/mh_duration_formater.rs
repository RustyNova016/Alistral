use core::fmt::Display;

use chrono::Duration;
use tuillez::extensions::chrono_exts::DurationExt as _;

pub struct MHDurationFormater(pub Option<Duration>);

impl Display for MHDurationFormater {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
