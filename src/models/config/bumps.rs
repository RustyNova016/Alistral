use chrono::{DateTime, Utc};
use derive_new::new;
use rust_decimal::{prelude::One, Decimal};
use serde::{Deserialize, Serialize};

use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

#[derive(Serialize, Deserialize, new, Default, Debug)]
pub struct Bump {
    pub(self) recording: RecordingMBID,
    pub(self) username: String,
    pub(self) value: Decimal,
    pub(self) expiration_date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, new, Default, Debug)]
pub struct BumpList(Vec<Bump>);

impl BumpList {
    pub fn add_bump(
        &mut self,
        recording: RecordingMBID,
        username: String,
        value: Decimal,
        expiration_date: DateTime<Utc>,
    ) {
        self.0.push(Bump {
            expiration_date,
            recording,
            value,
            username,
        });
    }

    #[deprecated]
    pub fn get_multiplier(&self, recording: &RecordingMBID) -> Decimal {
        let values = self
            .0
            .iter()
            .filter(|b| &b.recording == recording && b.expiration_date > Utc::now())
            .map(|b| b.value);
        let mut res = Decimal::one();

        for val in values {
            res *= val;
        }

        res
    }

    pub fn get_multiplier2(&self, recording_mbid: &str) -> Decimal {
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
