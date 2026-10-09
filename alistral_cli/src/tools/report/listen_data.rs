use core::cell::OnceCell;

use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;

pub(super) struct ReportListenData {
    all_time_stats: ListenStatisticsData,

    current_stats: OnceCell<ListenStatisticsData>,
    previous_stats: OnceCell<ListenStatisticsData>,

    current_all_time_stats: OnceCell<ListenStatisticsData>,
    previous_all_time_stats: OnceCell<ListenStatisticsData>,

    
}

impl ReportListenData {
    pub fn new(all_time_stats: ListenStatisticsData) -> Self {
        ReportListenData {
            all_time_stats,
            current_stats: OnceCell::new(),
            previous_stats: OnceCell::new(),
            current_all_time_stats: OnceCell::new(),
            previous_all_time_stats: OnceCell::new(),
        }
    }

    pub fn all_time_stats(&self) -> &ListenStatisticsData {
        &self.all_time_stats
    }

    pub fn current_stats(
        &self,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> &ListenStatisticsData {
        self.current_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(from, until)
        })
    }

    pub fn previous_stats(
        &self,
        from: DateTime<Utc>,
        until: DateTime<Utc>,
    ) -> &ListenStatisticsData {
        self.previous_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(from, until)
        })
    }

    pub fn current_all_time_stats(&self, until: DateTime<Utc>) -> &ListenStatisticsData {
        self.current_all_time_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(DateTime::UNIX_EPOCH, until)
        })
    }

    pub fn previous_all_time_stats(&self, until: DateTime<Utc>) -> &ListenStatisticsData {
        self.previous_all_time_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(DateTime::UNIX_EPOCH, until)
        })
    }
}
