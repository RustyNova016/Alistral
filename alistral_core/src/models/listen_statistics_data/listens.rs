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
