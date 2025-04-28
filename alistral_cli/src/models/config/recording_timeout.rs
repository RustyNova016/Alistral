use std::collections::HashMap;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;

use crate::models::config::ConfigFile;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingTimeoutConfig(HashMap<String, DateTime<Utc>>);

impl RecordingTimeoutConfig {
    pub fn set_timeout(&mut self, recording: &str, duration: Duration) {
        self.0.insert(recording.to_string(), Utc::now() + duration);
    }

    pub fn get_timed_out_recordings(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|(id, deadline)| {
                if &Utc::now() < deadline {
                    return Some(id);
                }
                None
            })
            .cloned()
            .collect_vec()
    }
}

impl ConfigFile for RecordingTimeoutConfig {
    fn file_name() -> &'static str {
        "timeouts.json"
    }
}
