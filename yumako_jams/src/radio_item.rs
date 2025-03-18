use core::ops::Deref;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use rust_decimal::Decimal;

use crate::modules::listen_data::ListenAction;
use crate::modules::scores::ScoreMerging;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioItem {
    pub recording: RecordingWithListens,
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

    pub fn set_listens(&mut self, listens: Vec<Listen>, action: ListenAction) {
        match action {
            ListenAction::Add => self.recording.insert_unique_listens_unchecked(listens),
            ListenAction::Remove => self
                .recording
                .retain(|l| listens.iter().all(|l2| l.id != l2.id)),
            ListenAction::Replace => self.recording.set_listens(listens.into()),
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

impl From<Recording> for RadioItem {
    fn from(value: Recording) -> Self {
        Self {
            recording: RecordingWithListens::new(value, Default::default()),
            score: Decimal::ZERO,
        }
    }
}
