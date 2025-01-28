use core::ops::Mul;

use chrono::Duration;
use collection::RecordingWithListensCollection;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::RowId;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::datastructures::listen_collection::traits::ListenCollectionReadable;
use crate::datastructures::listen_collection::ListenCollection;

use super::traits::ListenCollWithTime;
use super::EntityWithListens;

pub mod collection;

pub type RecordingWithListens = EntityWithListens<Recording, ListenCollection>;

impl RecordingWithListens {
    pub fn recording(&self) -> &Recording {
        &self.entity
    }

    /// Get the score for the underated radio
    pub fn get_underated_score(
        &self,
        user_listens: &RecordingWithListensCollection,
        global_listen_count: u64,
    ) -> Decimal {
        self.get_underrated_rank_score(user_listens)
            + self.get_underrated_listen_score(user_listens, global_listen_count)
    }

    fn get_underrated_rank_score(&self, user_listens: &RecordingWithListensCollection) -> Decimal {
        // Retrive the rank of the recording in the user listens
        let rank = user_listens.get_rank(&self.entity).unwrap_or(9999);

        // The recording vec scores if its between 0 and 999.
        Decimal::from(1000_u64.saturating_sub(rank.try_into().unwrap()))
            .checked_div(dec!(10))
            .unwrap_or(Decimal::ZERO)
    }

    fn get_underrated_listen_score(
        &self,
        user_listens: &RecordingWithListensCollection,
        global_listen_count: u64,
    ) -> Decimal {
        // Retrieve the all time listen count of the recording for the user
        let all_time_listen_count = user_listens
            .get_by_id(self.entity.get_row_id())
            .map(|r| r.listen_count())
            .unwrap_or(0);

        // Divide by the global listen count to get the fractions of listens made by the user
        // Then x100 to get the score to max out at 100 points
        Decimal::from(all_time_listen_count)
            .checked_div(Decimal::from(global_listen_count))
            // If `global_listen_count` is 0, it means that the stats haven't been processed yet on LB's side.
            // We'll be optimist and assume the user is the only listener
            .unwrap_or(dec!(1))
            // if `global_listen_count` is inferior to the stats, we cap at one.
            .min(Decimal::ONE)
            .saturating_mul(dec!(100))
    }
}

impl ListenCollWithTime for RecordingWithListens {
    fn get_time_listened(&self) -> Option<Duration> {
        self.entity
            .length_as_duration()
            .map(|dur| dur.mul(self.listen_count().try_into().unwrap()))
    }
}
