use std::fmt::Write;
use std::sync::LazyLock;

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

        let current_map = self.data.listens_per_month_current().await;
        let previous_map = self.data.listens_per_month_previous().await;

        let stats = if let Some(data) = current_map.get(&num) {
            data.recording_stats()
                .await
                .map(|stats| stats.to_owned())
                .unwrap_or_default()
        } else {
            Default::default()
        };

        let prev_month = if num == 1 {
            if let Some(data) = previous_map.get(&12) {
                data.recording_stats()
                    .await
                    .map(|stats| stats.to_owned())
                    .unwrap_or_default()
            } else {
                Default::default()
            }
        } else if let Some(data) = current_map.get(&(num - 1)) {
            data.recording_stats()
                .await
                .map(|stats| stats.to_owned())
                .unwrap_or_default()
        } else {
            Default::default()
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
            Self::top_recordings_with_cmp(stats.to_owned(), prev_month.to_owned()).await
        )
        .unwrap();

        out
    }
}
