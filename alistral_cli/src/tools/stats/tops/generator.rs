use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Recording;

pub struct TopGenerator {
    stats: ListenStatisticsData,

    from: Option<DateTime<Utc>>,
    until: Option<DateTime<Utc>>,
}

impl TopGenerator {
    pub fn new(
        stats: ListenStatisticsData,
        from: Option<DateTime<Utc>>,
        until: Option<DateTime<Utc>>,
    ) -> Self {
        Self { stats, from, until }
    }

    pub fn get_current_stats(&self) -> ListenStatisticsData {
        match &self.from {
            Some(from) => self
                .stats
                .clone_no_stats()
                .filter_listening_date(*from, self.until.unwrap_or_else(Utc::now)),
            None => self.stats.clone_no_stats(),
        }
    }

    pub fn get_previous_stats(&self) -> Option<ListenStatisticsData> {
        match &self.from {
            Some(from) => {
                let until = self.until.unwrap_or_else(Utc::now);
                let period = until - from;
                let previous_start = *from - period;

                let stats = self
                    .stats
                    .clone_no_stats()
                    .filter_listening_date(previous_start, *from);

                if stats.listens().is_empty() {
                    None
                } else {
                    Some(stats)
                }
            }
            None => None,
        }
    }

    pub async fn get_recording_stats(
        &self,
    ) -> (
        RecordingWithListensCollection,
        Option<RecordingWithListensCollection>,
        Vec<Recording>,
    ) {
        let cur_stats = self
            .get_current_stats()
            .recording_stats()
            .await
            .unwrap()
            .clone();
        let prev_stats = if let Some(prev_stats) = self.get_previous_stats() {
            Some(prev_stats.recording_stats().await.unwrap().clone())
        } else {
            None
        };

        let entities = cur_stats
            .iter_entities()
            .chain(prev_stats.iter().flat_map(|stats| stats.iter_entities()))
            .unique_by(|ent| ent.rowid())
            .cloned()
            .collect_vec();

        (cur_stats, prev_stats, entities)
    }
}
