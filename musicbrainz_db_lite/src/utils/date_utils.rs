use chrono::{NaiveDate, TimeZone, Utc};

pub fn date_to_timestamp(date: NaiveDate) -> Option<i64> {
    let date_time = date.and_hms_opt(0, 0, 0)?;
    let utc_date_time = Utc.from_utc_datetime(&date_time);
    Some(utc_date_time.timestamp())
}
