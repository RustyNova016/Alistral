use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;
use itertools::Itertools;

use crate::datastructures::listen_collection::ListenCollection;
use crate::models::listen_statistics_data::ListenStatisticsData;

impl ListenStatisticsData {
    /// Filter the inner listens to conform to a specific time period. This invalidate the inner statistics
    pub fn filter_listening_date(self, from: DateTime<Utc>, until: DateTime<Utc>) -> Self {
        let listens = self
            .listens
            .into_iter()
            .filter(|listen| {
                from <= listen.listened_at_as_datetime()
                    && listen.listened_at_as_datetime() <= until
            })
            .collect_vec();

        Self::new(self.client, ListenCollection::new(listens))
    }

    /// Split the stats into two [`ListenStatisticsData`]. One with the period desired, and the second one with the previous period, of same lenght
    pub fn comparison_split(&self, from: DateTime<Utc>, until: DateTime<Utc>) -> (Self, Self) {
        let period = until - from;
        let before_start = from - period;

        let now_stats = self.clone_no_stats().filter_listening_date(from, until);
        let before_stats = self
            .clone_no_stats()
            .filter_listening_date(before_start, from);

        (now_stats, before_stats)
    }

    /// Filter the listens on a specific year and month. This invalidate the inner statistics
    pub fn filter_on_year_month(self, year: i32, month: u32) -> Self {
        let listens = self
            .listens
            .into_iter()
            .filter(|listen| {
                let date = listen.listened_at_as_datetime();

                date.year() == year && date.month() == month
            })
            .collect_vec();

        Self::new(self.client, ListenCollection::new(listens))
    }
}
