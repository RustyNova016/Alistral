use core::ops::Deref;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use rust_decimal::Decimal;

use crate::modules::scores::ScoreMerging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioItem {
    recording: RecordingWithListens,
    pub score: Decimal,
}

impl RadioItem {
    pub fn set_score(&mut self, score: Decimal, merge: ScoreMerging) {
        match merge {
            ScoreMerging::Replace => self.score = score,
            ScoreMerging::Add => self.score += score,
            ScoreMerging::Sub => self.score -= score,
            ScoreMerging::Multiply => self.score *= score,
            ScoreMerging::Divide => self.score /= score,
        }
    }
}

impl Deref for RadioItem {
    type Target = RecordingWithListens;
    fn deref(&self) -> &Self::Target {
        &self.recording
    }
}

impl From<RecordingWithListens> for RadioItem {
    fn from(value: RecordingWithListens) -> Self {
        Self {
            recording: value,
            score: Decimal::ZERO,
        }
    }
}
