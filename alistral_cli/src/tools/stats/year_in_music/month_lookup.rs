use std::fmt::Write;
use std::sync::LazyLock;

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

        let stats = self
            .data
            .listens_per_month_current()
            .await
            .get(&num)
            .expect("Invalid month")
            .recording_stats()
            .await
            .unwrap();

        let prev_month = if num == 1 {
            self.data
                .listens_per_month_previous()
                .await
                .get(&12)
                .expect("Invalid month")
                .recording_stats()
                .await
                .unwrap()
        } else {
            self.data
                .listens_per_month_current()
                .await
                .get(&(num - 1))
                .expect("Invalid month")
                .recording_stats()
                .await
                .unwrap()
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
                prev_month.iter().cloned().collect_vec()
            )
            .await
        )
        .unwrap();

        out
    }
}
