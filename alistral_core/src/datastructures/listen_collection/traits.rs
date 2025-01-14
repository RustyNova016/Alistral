use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use rust_decimal::prelude::FromPrimitive as _;
use rust_decimal::Decimal;

use super::ListenCollection;

/// Trait implemented for all the entities that hold listens, and those listens can be read
pub trait ListenCollectionReadable {
    /// Return the number of listens in the collection
    fn listen_count(&self) -> usize {
        self.iter_listens().unique_by(|l| l.id).collect_vec().len()
    }

    fn has_no_listens(&self) -> bool {
        self.listen_count() == 0
    }

    /// Iterate over all the listens. They may not be deduplicated
    fn iter_listens(&self) -> impl Iterator<Item = &Listen>;

    // --- Listen getters

    /// Returns the oldest listen in the collection.
    fn get_oldest_listen(&self) -> Option<&Listen> {
        self.iter_listens().min_by_key(|listen| listen.listened_at)
    }

    /// Returns the latest listen in the collection.
    fn get_latest_listen(&self) -> Option<&Listen> {
        self.iter_listens().max_by_key(|listen| listen.listened_at)
    }

    // --- Interogations ---

    fn is_empty(&self) -> bool {
        self.iter_listens().next().is_none()
    }

    /// The date of the oldest listen
    fn oldest_listen_date(&self) -> Option<DateTime<Utc>> {
        self.get_oldest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    /// The date of the latest listen
    fn latest_listen_date(&self) -> Option<DateTime<Utc>> {
        self.get_latest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    fn average_duration_between_listens(&self) -> Duration {
        // If the recording haven't been listened to, then the average time is zero
        if self.listen_count() < 2 {
            return Duration::zero();
        }

        let duration_between_first_and_last = self
            .latest_listen_date()
            .expect("There's at least two listens")
            - self
                .oldest_listen_date()
                .expect("There's at least two listens");

        duration_between_first_and_last
            .checked_div(self.listen_count() as i32)
            .unwrap_or_else(Duration::zero)
    }

    fn average_duration_between_listens_and_date(&self, date: DateTime<Utc>) -> Duration {
        // If the recording haven't been listened to, then the average time is zero
        if self.is_empty() {
            return Duration::zero();
        }

        let duration_between_first_and_last = date
            - self
                .oldest_listen_date()
                .expect("There's at least a listen");

        duration_between_first_and_last
            .checked_div(self.listen_count() as i32)
            .unwrap_or_else(Duration::zero)
    }

    fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.latest_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    fn overdue_by(&self) -> Duration {
        self.overdue_by_at(&Utc::now())
    }

    fn overdue_by_at(&self, date: &DateTime<Utc>) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| *date - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    fn overdue_factor(&self) -> Decimal {
        self.overdue_factor_at(&Utc::now())
    }

    fn overdue_factor_at(&self, date: &DateTime<Utc>) -> Decimal {
        Decimal::from_i64(self.overdue_by_at(date).num_seconds())
            .unwrap()
            .checked_div(
                Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap(),
            )
            .unwrap_or(Decimal::NEGATIVE_ONE)
    }

    /// Get the number of listens estimated to be made for a time period
    fn get_listen_rate(&self, period: Duration) -> Option<Decimal> {
        Decimal::from(period.num_seconds()).checked_div(Decimal::from(
            self.average_duration_between_listens().num_seconds(),
        ))
    }
}

impl ListenCollectionReadable for ListenCollection {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.data.iter()
    }
}
