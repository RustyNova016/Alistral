use std::collections::HashMap;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use chrono::Datelike;
use itertools::Itertools;

use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn discoveries_per_year(&self) -> &HashMap<i32, Vec<RecordingWithListens>> {
        self.discoveries_per_year
            .get_or_init(async {
                let recordings = self.user_data.recording_stats().await.unwrap();
                generate_discovery_years(recordings.iter().cloned().collect_vec())
            })
            .await
    }

    pub async fn discovery_years_current(&self) -> &Vec<RecordingWithListens> {
        self.discovery_years_current
            .get_or_init(async {
                discovery_stats_for_year(self.discoveries_per_year().await, self.year)
            })
            .await
    }

    pub async fn discovery_years_previous(&self) -> &Vec<RecordingWithListens> {
        self.discovery_years_previous
            .get_or_init(async {
                discovery_stats_for_year(self.discoveries_per_year().await, self.year - 1)
            })
            .await
    }
}

fn generate_discovery_years(
    recordings: Vec<RecordingWithListens>,
) -> HashMap<i32, Vec<RecordingWithListens>> {
    let mut map: HashMap<i32, Vec<RecordingWithListens>> = HashMap::new();

    for rec in recordings {
        let year = rec
            .oldest_listen_date()
            .map(|date| date.year())
            .unwrap_or(0);
        map.entry(year).or_default().push(rec);
    }

    map
}

fn discovery_stats_for_year(
    discoveries: &HashMap<i32, Vec<RecordingWithListens>>,
    year: i32,
) -> Vec<RecordingWithListens> {
    let Some(discoveries) = discoveries.get(&year) else {
        return Vec::new();
    };

    let mut out = Vec::new();

    for rec in discoveries {
        let listens = rec
            .iter_listens()
            .filter(|lis| lis.listened_at_as_datetime().year() == year)
            .cloned()
            .collect_vec();

        if !listens.is_empty() {
            out.push(RecordingWithListens::new(
                rec.entity().clone(),
                listens.into(),
            ));
        }
    }

    out
}
