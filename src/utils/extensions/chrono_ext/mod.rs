use chrono::DateTime;
use chrono::Duration;
use chrono::OutOfRangeError;
use chrono::Utc;
use extend::ext;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[ext]
pub impl Duration {
    fn from_human_string(value: &str) -> color_eyre::Result<Duration> {
        let human_dur: humantime::Duration = value.parse()?;
        Ok(Duration::from_std(*human_dur)?)
    }

    fn to_humantime(self) -> Result<humantime::Duration, OutOfRangeError> {
        Ok(humantime::Duration::from(self.to_std()?))
    }

    #[must_use]
    fn floor_to_minute(self) -> Self {
        Self::minutes(self.num_minutes())
    }

    fn deci_minutes(&self) -> Decimal {
        Decimal::from(self.num_seconds()) / dec!(60)
    }

    fn deci_hours(&self) -> Decimal {
        self.deci_minutes() / dec!(60)
    }

    fn format_hh_mm(&self) -> String {
        let minutes = self.num_minutes().rem_euclid(60);
        let formatted_minutes = if minutes < 10 {
            format!("0{minutes}")
        } else {
            format!("{minutes}")
        };

        format!("{}:{formatted_minutes}", self.num_hours())
    }
}

#[ext]
pub impl DateTime<Utc> {
    #[must_use]
    fn floor_to_second(self) -> Self {
        Self::from_timestamp(self.timestamp(), 0).unwrap()
    }
}
