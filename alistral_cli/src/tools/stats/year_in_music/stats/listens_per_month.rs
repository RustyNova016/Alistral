use std::collections::HashMap;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::Datelike;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::tools::stats::year_in_music::YimReportData;

impl YimReportData {
    pub async fn listens_per_month_current(&self) -> &HashMap<u32, ListenStatisticsData> {
        self.listens_per_month_current
            .get_or_init(async { generate_listens_per_month(self.current.clone_no_stats()).await })
            .await
    }

    pub async fn listens_per_month_previous(&self) -> &HashMap<u32, ListenStatisticsData> {
        self.listens_per_month_previous
            .get_or_init(async { generate_listens_per_month(self.previous.clone_no_stats()).await })
            .await
    }
}

async fn generate_listens_per_month(
    listens: ListenStatisticsData,
) -> HashMap<u32, ListenStatisticsData> {
    let mut mapping: HashMap<u32, Vec<Listen>> = HashMap::new();

    for listen in listens.listens().iter_listens() {
        let listen_date = listen.listened_at_as_datetime();

        mapping
            .entry(listen_date.month())
            .or_default()
            .push(listen.to_owned());
    }

    let mut out: HashMap<u32, ListenStatisticsData> = HashMap::new();

    for (date, list) in mapping {
        out.insert(
            date,
            ListenStatisticsData::new(listens.client().clone(), list.into()),
        );
    }

    out
}
