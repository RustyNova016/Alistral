use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use collection::RecordingWithListensCollection;
use derive_getters::Getters;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use musicbrainz_db_lite::models::musicbrainz::recording::Recording;
use musicbrainz_db_lite::models::musicbrainz::user::User;
use musicbrainz_db_lite::RowId;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use serde::Deserialize;
use serde::Serialize;

use crate::api::listenbrainz::global_listen_counts::get_global_listen_counts;
use crate::database::listenbrainz::prefetching::prefetch_recordings_of_listens;
use crate::datastructures::listen_collection::traits::ListenCollectionLike;
use crate::datastructures::listen_collection::ListenCollection;

use super::impl_entity_with_listens;

pub mod collection;
pub mod info;
pub mod radios;

#[derive(Debug, Clone, PartialEq, Eq, Getters, Deserialize, Serialize)]
pub struct RecordingWithListens {
    pub(self) recording: Recording,
    listens: ListenCollection,
}

impl RecordingWithListens {
    pub fn new(recording: Recording, listens: ListenCollection) -> Self {
        Self { listens, recording }
    }

    pub async fn from_listencollection(
        conn: &mut sqlx::SqliteConnection,
        listens: ListenCollection,
    ) -> Result<RecordingWithListensCollection, crate::Error> {
        // If empty, early return
        if listens.is_empty() {
            return Ok(Default::default());
        }

        // Prefetch the missing data
        let user_name = listens
            .first()
            .expect("At least one listen should be there")
            .user
            .clone();

        let user = User::find_by_name(conn, &user_name)
            .await?
            .ok_or(crate::Error::MissingUserError(user_name.clone()))?;

        prefetch_recordings_of_listens(conn, user.id, &listens.data).await?;

        // Get all the data from the DB
        let joins = Listen::get_recordings_as_batch(conn, user.id, listens.data).await?;

        // Convert into structs
        let mut out = HashMap::new();

        for (_, (listen, recordings)) in joins {
            for recording in recordings {
                out.entry(recording.get_row_id())
                    .or_insert_with(|| Self {
                        recording,
                        listens: ListenCollection::default(),
                    })
                    .push(listen.clone());
            }
        }

        Ok(out.into())
    }

    pub fn push(&mut self, listen: Listen) {
        self.listens.push(listen);
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_oldest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_latest_listen()
            .map(|listen| listen.listened_at_as_datetime())
    }

    /// Return the amount of time this recording having known about
    pub fn known_for(&self) -> Option<Duration> {
        self.first_listen_date()
            .map(|discovery| Utc::now() - discovery)
    }

    pub fn average_duration_between_listens(&self) -> Duration {
        // If the recording haven't been listened to, then the average time is zero
        if self.listen_count() < 2 {
            return Duration::zero();
        }

        let duration_between_first_and_last = self
            .last_listen_date()
            .expect("There's at least two listens")
            - self
                .first_listen_date()
                .expect("There's at least two listens");

        duration_between_first_and_last
            .checked_div(self.listen_count() as i32)
            .unwrap_or_else(Duration::zero)
    }

    pub fn average_duration_between_listens_and_date(&self, date: DateTime<Utc>) -> Duration {
        // If the recording haven't been listened to, then the average time is zero
        if self.is_empty() {
            return Duration::zero();
        }

        let duration_between_first_and_last =
            date - self.first_listen_date().expect("There's at least a listen");

        duration_between_first_and_last
            .checked_div(self.len() as i32)
            .unwrap_or_else(Duration::zero)
    }

    pub fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.last_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    pub fn overdue_by(&self) -> Duration {
        self.overdue_by_at(&Utc::now())
    }

    pub fn overdue_by_at(&self, date: &DateTime<Utc>) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| *date - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    pub fn overdue_factor(&self) -> Decimal {
        self.overdue_factor_at(&Utc::now())
    }

    pub fn overdue_factor_at(&self, date: &DateTime<Utc>) -> Decimal {
        Decimal::from_i64(self.overdue_by_at(date).num_seconds())
            .unwrap()
            .checked_div(
                Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap(),
            )
            .unwrap_or(Decimal::NEGATIVE_ONE)
    }

    pub fn is_listened(&self) -> bool {
        !self.listens.data.is_empty()
    }

    /*     pub async fn underated_score_single(&self) -> color_eyre::Result<Decimal> {
        Ok(self
            .listens()
            .get_underrated_recordings()
            .await?
            .first()
            .expect("Recording should have a score")
            .0)
    } */

    /// Get the number of listens estimated to be made for a time period
    pub fn get_listen_rate(&self, period: Duration) -> Option<Decimal> {
        Decimal::from(period.num_seconds()).checked_div(Decimal::from(
            self.average_duration_between_listens().num_seconds(),
        ))
    }

    pub fn merge(&mut self, other: Self) {
        if self.recording.id != other.recording.id {
            #[cfg(debug_assertions)] // This is an awkward situation. Let's crash in debug to catch those cases
            panic!("Tried to merge two different recordings");

            #[cfg(not(debug_assertions))]
            return;
        }

        self.listens.merge_by_index(other.listens);
    }

    pub async fn get_global_listen_count(&self) -> Result<u64, crate::Error> {
        let counts = get_global_listen_counts(&[self.recording.mbid.to_string()]).await?;
        let Some(count) = counts.first() else {
            return Ok(0);
        };
        Ok(count.total_listen_count.unwrap_or(0))
    }

    /// Return the total time the recording has been listened at
    pub fn get_time_listened(&self) -> Option<Duration> {
        self.recording()
            .length_as_duration()
            .map(|dur| dur * self.listen_count().try_into().unwrap())
    }
}

impl_entity_with_listens!(RecordingWithListens);

impl ListenCollectionLike for RecordingWithListens {
    fn iter_listens(&self) -> impl Iterator<Item = &Listen> {
        self.listens.iter_listens()
    }
}
