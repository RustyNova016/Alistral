use chrono::{DateTime, Utc};
use rust_decimal::{Decimal, prelude::One};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Bump {
    pub(self) recording: String,
    pub(self) username: String,
    pub(self) value: Decimal,
    pub(self) expiration_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BumpList(Vec<Bump>);

impl BumpList {
    pub fn add_bump(
        &mut self,
        recording: String,
        username: String,
        value: Decimal,
        expiration_date: DateTime<Utc>,
    ) {
        let now = Utc::now();
        self.0.retain(|bump| bump.expiration_date > now);

        self.0.push(Bump {
            expiration_date,
            recording,
            value,
            username,
        });
    }

    pub fn get_multiplier(&self, recording_mbid: &str) -> Decimal {
        let values = self
            .0
            .iter()
            .filter(|b| b.recording.as_str() == recording_mbid && b.expiration_date > Utc::now())
            .map(|b| b.value);
        let mut res = Decimal::one();

        for val in values {
            res *= val;
        }

        res
    }
}
