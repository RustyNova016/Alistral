use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;

use crate::models::statistics_buckets::listen_statistics_period::ListenStatisticsPeriod;

pub struct ListenStatisticsPeriodComparison {
    pub current: ListenStatisticsPeriod,
    pub previous: ListenStatisticsPeriod,
}

impl ListenStatisticsPeriodComparison {
    pub fn new(data: ListenStatisticsData, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        let period_duration = end - start;
        let previous_start_time = (start - period_duration).to_utc();

        Self {
            current: ListenStatisticsPeriod::new(data.clone_no_stats(), start, end),
            previous: ListenStatisticsPeriod::new(data, previous_start_time, start),
        }
    }
}
