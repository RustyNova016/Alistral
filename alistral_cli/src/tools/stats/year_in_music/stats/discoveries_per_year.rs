use std::collections::HashMap;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Datelike;
use itertools::Itertools;

use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn discovery_years_current(&self) -> &HashMap<u64, Vec<RecordingWithListens>> {
        self.discovery_years_current
            .get_or_init(async {
                let recordings = self.user_data.recording_stats().await.unwrap();
                generate_discoveries_per_year(recordings.iter().cloned().collect_vec()).await
            })
            .await
    }

    pub async fn discovery_years_previous(&self) -> &HashMap<u64, Vec<RecordingWithListens>> {
        self.discovery_years_previous
            .get_or_init(async {
                let recordings = self.user_data.recording_stats().await.unwrap();
                generate_discoveries_per_year(recordings.iter().cloned().collect_vec()).await
            })
            .await
    }
}

async fn generate_discoveries_per_year(
    recordings: Vec<RecordingWithListens>,
) -> HashMap<u64, Vec<RecordingWithListens>> {
    let mut out: HashMap<u64, Vec<RecordingWithListens>> = HashMap::new();

    for rec in recordings {
        let Some(oldest_listen) = rec.listens().oldest_listen_date() else {
            continue;
        };

        out.entry(oldest_listen.year() as u64)
            .or_default()
            .push(rec);
    }

    out
}
