use core::fmt::Display;

use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use tuillez::extensions::chrono_exts::DateTimeUtcExt as _;

pub struct LookupLocalDate(pub Option<DateTime<Utc>>);

impl Display for LookupLocalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(date) = &self.0 else {
            write!(f, "-")?;
            return Ok(());
        };

        write!(f, "{}", date.floor_to_second().with_timezone(&Local))?;

        Ok(())
    }
}
