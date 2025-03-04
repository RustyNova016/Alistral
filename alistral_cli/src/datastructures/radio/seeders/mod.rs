use chrono::DateTime;
use chrono::Utc;
use macon::Builder;

pub mod listens;

#[derive(Debug, Clone, Default, Builder)]
pub struct SeederSettings {
    #[builder(Option=!)]
    min_listened_at: Option<DateTime<Utc>>,

    #[builder(Option=!)]
    max_listened_at: Option<DateTime<Utc>>,

    min_listen_per_recording: u64,
}
