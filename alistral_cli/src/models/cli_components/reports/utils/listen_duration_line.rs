use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;

pub(in crate::models::cli_components::reports) fn listen_duration_line(
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    current_listen_duration: Option<TimeDelta>,
    previous_listen_duration: Option<TimeDelta>,
) -> String {
    let mut out = String::new();

    

    out
}
