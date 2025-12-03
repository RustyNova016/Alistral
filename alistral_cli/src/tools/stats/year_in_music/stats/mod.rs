use std::collections::HashMap;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use async_once_cell::OnceCell;
use chrono::Duration;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;
use tracing::instrument;
use tuillez::pg_counted;
use tuillez::pg_inc;

use crate::models::hour_min::HourMinute;
use crate::tools::stats::year_in_music::YimReport;

pub mod discoveries_per_year;
pub mod listen_per_hourmin;
pub mod listen_with_duration;
pub mod listens_per_month;
pub mod release_years;

impl YimReport {
    pub async fn num_listens_in_year(&self) -> usize {
        self.data.current.listens().listen_count()
    }
}

pub(super) struct YimReportData {
    user_data: ListenStatisticsData,

    pub current: ListenStatisticsData,
    pub previous: ListenStatisticsData,

    listens_with_duration_current: OnceCell<Vec<(Listen, Duration)>>,
    listens_with_duration_previous: OnceCell<Vec<(Listen, Duration)>>,

    listen_per_hourmin_current: OnceCell<HashMap<HourMinute, Vec<Listen>>>,
    listen_per_hourmin_previous: OnceCell<HashMap<HourMinute, Vec<Listen>>>,

    release_years_current: OnceCell<HashMap<u64, Vec<RecordingWithListens>>>,
    release_years_previous: OnceCell<HashMap<u64, Vec<RecordingWithListens>>>,

    discovery_years_current: OnceCell<HashMap<u64, Vec<RecordingWithListens>>>,
    discovery_years_previous: OnceCell<HashMap<u64, Vec<RecordingWithListens>>>,

    listens_per_month_current: OnceCell<HashMap<u32, ListenStatisticsData>>,
    listens_per_month_previous: OnceCell<HashMap<u32, ListenStatisticsData>>,
}

impl YimReportData {
    pub fn new(
        user_data: ListenStatisticsData,

        current: ListenStatisticsData,
        previous: ListenStatisticsData,
    ) -> Self {
        Self {
            user_data,
            current,
            previous,

            listens_with_duration_current: OnceCell::new(),
            listens_with_duration_previous: OnceCell::new(),

            listen_per_hourmin_current: OnceCell::new(),
            listen_per_hourmin_previous: OnceCell::new(),

            release_years_current: OnceCell::new(),
            release_years_previous: OnceCell::new(),

            discovery_years_current: OnceCell::new(),
            discovery_years_previous: OnceCell::new(),

            listens_per_month_current: OnceCell::new(),
            listens_per_month_previous: OnceCell::new(),
        }
    }

    #[instrument(skip(self), fields(indicatif.pb_show = tracing::field::Empty))]
    pub async fn prefetch(&self) {
        pg_counted!(18, "Fetching data");

        self.listens_with_duration_current().await;
        pg_inc!();
        self.listens_with_duration_previous().await;
        pg_inc!();

        self.listen_per_hourmin_current().await;
        pg_inc!();
        self.listen_per_hourmin_previous().await;
        pg_inc!();

        self.release_years_current().await;
        pg_inc!();
        self.release_years_previous().await;
        pg_inc!();

        self.discovery_years_current().await;
        pg_inc!();
        self.discovery_years_previous().await;
        pg_inc!();

        self.current
            .artists_stats()
            .await
            .expect("Couldn't fetch artists for the current year");
        pg_inc!();
        self.previous
            .artists_stats()
            .await
            .expect("Couldn't fetch artists for the previous year");
        pg_inc!();

        self.current
            .release_stats()
            .await
            .expect("Couldn't fetch release for the current year");
        pg_inc!();
        self.previous
            .release_stats()
            .await
            .expect("Couldn't fetch release for the previous year");
        pg_inc!();

        self.current
            .release_group_stats()
            .await
            .expect("Couldn't fetch release groups for the current year");
        pg_inc!();
        self.previous
            .release_group_stats()
            .await
            .expect("Couldn't fetch release groups for the previous year");
        pg_inc!();

        self.current
            .label_stats()
            .await
            .expect("Couldn't fetch labels for the current year");
        pg_inc!();
        self.previous
            .label_stats()
            .await
            .expect("Couldn't fetch labels for the previous year");
        pg_inc!();

        for listens in self.listens_per_month_current().await.values() {
            listens
                .recording_stats()
                .await
                .expect("Couldn't fetch recording for the current year");
        }
        pg_inc!();

        for listens in self.listens_per_month_previous().await.values() {
            listens
                .recording_stats()
                .await
                .expect("Couldn't fetch recording for the previous year");
        }
        pg_inc!();
    }
}
