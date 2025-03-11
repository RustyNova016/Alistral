use chrono::DateTime;
use chrono::Duration;
use chrono::OutOfRangeError;
use chrono::Utc;
use extend::ext;
use jiff::Unit;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use thiserror::Error;

#[ext]
pub impl Duration {
    fn from_human_string(value: &str) -> Result<Duration, TimeError> {
        let human_dur: jiff::Span = value.parse().map_err(TimeError::ParseError)?;
        let num_secs = human_dur
            .total(Unit::Second)
            .map_err(TimeError::ConvertError)?;
        let num_secs = num_secs.round() as i64;

        Ok(Duration::new(num_secs, 0).unwrap())
    }

    fn to_humantime(self) -> Result<jiff::Span, OutOfRangeError> {
        Ok(jiff::Span::new().seconds(self.num_seconds()))
    }

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
    fn floor_to_second(self) -> Self {
        Self::from_timestamp(self.timestamp(), 0).unwrap()
    }
}

#[derive(Error, Debug)]
//#[expect(clippy::enum_variant_names)]
pub enum TimeError {
    #[error(transparent)]
    ParseError(jiff::Error),

    #[error(transparent)]
    ConvertError(jiff::Error),
}
