use std::collections::HashMap;

use alistral_core::AlistralClient;
use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use chrono::Datelike;
use itertools::Itertools;

use crate::ALISTRAL_CLIENT;
use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn release_years_current(&self) -> &HashMap<u64, Vec<RecordingWithListens>> {
        self.release_years_current
            .get_or_init(async {
                let recordings = self.current.recording_stats().await.unwrap();
                generate_new_releases(
                    &ALISTRAL_CLIENT.core,
                    recordings.iter().cloned().collect_vec(),
                )
                .await
            })
            .await
    }

    pub async fn release_years_previous(&self) -> &HashMap<u64, Vec<RecordingWithListens>> {
        self.release_years_previous
            .get_or_init(async {
                let recordings = self.previous.recording_stats().await.unwrap();
                generate_new_releases(
                    &ALISTRAL_CLIENT.core,
                    recordings.iter().cloned().collect_vec(),
                )
                .await
            })
            .await
    }
}

async fn generate_new_releases(
    client: &AlistralClient,
    recordings: Vec<RecordingWithListens>,
) -> HashMap<u64, Vec<RecordingWithListens>> {
    let mut out: HashMap<u64, Vec<RecordingWithListens>> = HashMap::new();

    for rec in recordings {
        let Some(release_date) = rec
            .entity()
            .first_release_date_or_fetch(client.musicbrainz_db.clone())
            .await
            .expect("Error while getting the recording's release date")
        else {
            continue;
        };

        out.entry(release_date.year() as u64).or_default().push(rec);
    }

    out
}
