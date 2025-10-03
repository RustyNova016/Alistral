use chrono::DateTime;
use chrono::Utc;

pub mod listens;

#[derive(Debug, Clone, Default, bon::Builder)]
pub struct SeederSettings {
    min_listened_at: Option<DateTime<Utc>>,

    max_listened_at: Option<DateTime<Utc>>,

    min_listen_per_recording: u64,
}
