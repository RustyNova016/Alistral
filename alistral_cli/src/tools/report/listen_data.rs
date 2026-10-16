use core::cell::OnceCell;

use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Utc;

pub struct ReportListenData {
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    previous_start_time: DateTime<Utc>,

    all_time_stats: ListenStatisticsData,

    current_stats: OnceCell<ListenStatisticsData>,
    previous_stats: OnceCell<ListenStatisticsData>,

    current_all_time_stats: OnceCell<ListenStatisticsData>,
    previous_all_time_stats: OnceCell<ListenStatisticsData>,
}

impl ReportListenData {
    pub fn new(
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        previous_start_time: DateTime<Utc>,
        all_time_stats: ListenStatisticsData,
    ) -> Self {
        ReportListenData {
            end_time,
            previous_start_time,
            start_time,
            all_time_stats,
            current_stats: OnceCell::new(),
            previous_stats: OnceCell::new(),
            current_all_time_stats: OnceCell::new(),
            previous_all_time_stats: OnceCell::new(),
        }
    }

    pub fn start_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        self.end_time
    }

    pub fn previous_start_time(&self) -> DateTime<Utc> {
        self.previous_start_time
    }

    pub fn all_time_stats(&self) -> &ListenStatisticsData {
        &self.all_time_stats
    }

    pub fn current_stats(&self) -> &ListenStatisticsData {
        self.current_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(self.start_time, self.end_time)
        })
    }

    pub fn previous_stats(&self) -> &ListenStatisticsData {
        self.previous_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(self.previous_start_time, self.start_time)
        })
    }

    pub fn current_all_time_stats(&self) -> &ListenStatisticsData {
        self.current_all_time_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(DateTime::UNIX_EPOCH, self.end_time)
        })
    }

    pub fn previous_all_time_stats(&self) -> &ListenStatisticsData {
        self.previous_all_time_stats.get_or_init(|| {
            self.all_time_stats
                .clone_no_stats()
                .filter_listening_date(DateTime::UNIX_EPOCH, self.start_time)
        })
    }
}
