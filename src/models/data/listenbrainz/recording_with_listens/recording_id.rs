use crate::models::data::listenbrainz::listen::collection::ListenCollection;
use crate::models::data::musicbrainz::recording::mbid::RecordingMBID;

use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use derive_getters::Getters;
use itertools::Itertools;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[derive(Debug, Clone, PartialEq, Eq, Getters)]
#[deprecated]
pub struct RecordingIDWithListens {
    recording_id: RecordingMBID,
    listens: ListenCollection,
}

impl RecordingIDWithListens {
    pub fn new(recording_id: RecordingMBID, listens: ListenCollection) -> Self {
        //TODO: Perf Testing
        assert!(
            listens.has_only_recording(&recording_id),
            "Tried to insert a listen list that contain a listen from another recording"
        );

        Self {
            recording_id,
            listens,
        }
    }

    pub fn new_from_unfiltered(recording_id: RecordingMBID, listens: &ListenCollection) -> Self {
        let filtered = listens.get_listens_of_recording(&recording_id);

        Self {
            recording_id,
            listens: filtered,
        }
    }

    pub async fn all_from_unfiltered(listens: &ListenCollection) -> color_eyre::Result<Vec<Self>> {
        let recordings = listens.get_listened_recordings_mbids().await?;

        Ok(recordings
            .into_iter()
            .map(|rec| Self::new_from_unfiltered(rec, listens))
            .collect_vec())
    }

    pub fn first_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_oldest_listen()
            .map(|listen| *listen.get_listened_at())
    }

    pub fn last_listen_date(&self) -> Option<DateTime<Utc>> {
        self.listens
            .get_latest_listen()
            .map(|listen| *listen.get_listened_at())
    }

    pub fn listen_count(&self) -> usize {
        self.listens.len()
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

    pub fn estimated_date_of_next_listen(&self) -> Option<DateTime<Utc>> {
        self.last_listen_date()
            .map(|listen_date| listen_date + self.average_duration_between_listens())
    }

    pub fn overdue_by(&self) -> Duration {
        self.estimated_date_of_next_listen()
            .map(|next_listen| Utc::now() - next_listen)
            .unwrap_or_else(Duration::zero)
    }

    pub fn is_listened(&self) -> bool {
        !self.listens.is_empty()
    }



    pub fn overdue_score(&self) -> Decimal {
        Decimal::from_i64(self.overdue_by().num_seconds()).unwrap()
            / Decimal::from_i64(self.average_duration_between_listens().num_seconds()).unwrap()
    }
}
