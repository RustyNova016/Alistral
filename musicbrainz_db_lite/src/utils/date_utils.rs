use chrono::{NaiveDate, TimeZone, Utc};
use musicbrainz_rs::entity::date_string::DateString;

pub fn date_to_timestamp(date: NaiveDate) -> Option<i64> {
    let date_time = date.and_hms_opt(0, 0, 0)?;
    let utc_date_time = Utc.from_utc_datetime(&date_time);
    Some(utc_date_time.timestamp())
}

pub fn date_string_to_timestamp(date: DateString) -> Option<i64> {
    if date.0.is_empty() {
        return None;
    }

    date_to_timestamp(
        date.into_naive_date(1, 1)
            .expect("DateString doesn't contain a date"),
    )
}
