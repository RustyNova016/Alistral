use std::fmt::Write;
use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use chrono::DateTime;
use chrono::Datelike;
use chrono::Local;
use chrono::Months;
use itertools::Itertools;

use crate::datastructures::cli_formating::title::Heading1;
use crate::tools::stats::year_in_music::YimReport;
use crate::utils::cli::await_next;

static MONTHS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]
});

impl YimReport {
    pub async fn all_month_lookup_page(&self) {
        for i in 1..13 {
            println!("{}", self.month_lookup_page(i).await);
            println!("[Press enter to continue]");
            await_next();
        }
    }

    async fn month_lookup_page(&self, num: u32) -> String {
        let mut out = String::new();
        let stats = self.get_month_stats(self.year_start, num).await;
        let prev_year = if num == 1 {
            self.get_month_stats(self.year_start.with_year(self.year - 1).unwrap(), 12)
                .await
        } else {
            self.get_month_stats(self.year_start, num - 1).await
        };

        writeln!(
            out,
            "{}",
            Heading1(format!(
                "Looking back on {} 📅",
                MONTHS.get(num as usize - 1).unwrap()
            ))
        )
        .unwrap();

        writeln!(out, "Here's your top listens of this month:").unwrap();
        writeln!(out).unwrap();
        writeln!(
            out,
            "{}",
            Self::top_recordings_with_cmp(
                stats.iter().cloned().collect_vec(),
                prev_year.iter().cloned().collect_vec()
            )
            .await
        )
        .unwrap();

        out
    }

    async fn get_month_stats(
        &self,
        year_start: DateTime<Local >,
        month: u32,
    ) -> RecordingWithListensCollection {
        let month_start = year_start.with_month(month).unwrap();
        let month_end = month_start.checked_add_months(Months::new(1)).unwrap();

        let listens = self
            .full_user_stats
            .clone_no_stats()
            .filter_listening_date(month_start.into(), month_end.into());

        listens.recording_stats().await.unwrap().to_owned()
    }
}
