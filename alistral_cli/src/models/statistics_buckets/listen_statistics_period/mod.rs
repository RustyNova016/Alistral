use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;

pub struct ListenStatisticsPeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,

    pub data: ListenStatisticsData,
}

impl ListenStatisticsPeriod {
    pub fn new(data: ListenStatisticsData, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let data = data.filter_listening_date(start, end);

        Self { start, end, data }
    }
}
