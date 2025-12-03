use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::Duration;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::tools::stats::year_in_music::stats::YimReportData;

impl YimReportData {
    pub async fn listens_with_duration_current(&self) -> &Vec<(Listen, Duration)> {
        self.listens_with_duration_current
            .get_or_init(async {
                let recordings = self.current.recording_stats().await.unwrap();
                generate_listen_with_duration(recordings)
            })
            .await
    }

    pub async fn listens_with_duration_previous(&self) -> &Vec<(Listen, Duration)> {
        self.listens_with_duration_previous
            .get_or_init(async {
                let recordings = self.previous.recording_stats().await.unwrap();
                generate_listen_with_duration(recordings)
            })
            .await
    }
}

fn generate_listen_with_duration(
    recordings: &RecordingWithListensCollection,
) -> Vec<(Listen, Duration)> {
    recordings
        .iter()
        .flat_map(|rec| {
            let duration = rec.entity().length_as_duration().unwrap_or_default();

            rec.iter_listens()
                .map(move |listen| (listen.to_owned(), duration))
        })
        .collect_vec()
}
