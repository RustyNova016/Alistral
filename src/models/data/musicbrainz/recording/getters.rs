use chrono::Duration;

use super::Recording;

impl Recording {
    pub fn get_duration(&self) -> Option<Duration> {
        self.length.and_then(|length| {
            Duration::new(
                length.div_euclid(1000).into(),
                length.rem_euclid(1000),
            )
        })
    }
}
