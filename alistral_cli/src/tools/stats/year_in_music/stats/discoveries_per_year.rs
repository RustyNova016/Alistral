use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Datelike;
use itertools::Itertools;

use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn discovery_years_current(&self) -> &Vec<RecordingWithListens> {
        self.discovery_years_current
            .get_or_init(async {
                let recordings = self.user_data.recording_stats().await.unwrap();
                generate_discoveries_of_year(recordings.iter().cloned().collect_vec(), self.year)
                    .await
            })
            .await
    }

    pub async fn discovery_years_previous(&self) -> &Vec<RecordingWithListens> {
        self.discovery_years_previous
            .get_or_init(async {
                let recordings = self.user_data.recording_stats().await.unwrap();
                generate_discoveries_of_year(
                    recordings.iter().cloned().collect_vec(),
                    self.year - 1,
                )
                .await
            })
            .await
    }
}

async fn generate_discoveries_of_year(
    recordings: Vec<RecordingWithListens>,
    max_year: i32,
) -> Vec<RecordingWithListens> {
    let mut out = Vec::new();

    for rec in recordings {
        let Some(oldest_listen) = rec.listens().oldest_listen_date() else {
            continue;
        };

        if oldest_listen.year() > max_year {
            continue;
        }

        let listens = rec
            .iter_listens()
            .filter(|lis| lis.listened_at_as_datetime().year() == max_year)
            .cloned()
            .collect_vec();

        out.push(RecordingWithListens::new(
            rec.entity().clone(),
            listens.into(),
        ));
    }

    out
}
