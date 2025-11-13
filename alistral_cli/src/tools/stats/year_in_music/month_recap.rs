use std::collections::HashMap;
use std::fmt::Write;
use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use charchart::bar_graph::BarGraph;
use charchart::bar_graph::colors::Color;
use charchart::bar_graph::data::Data;
use chrono::Datelike;
use chrono::Months;
use tuillez::extensions::chrono_exts::DurationExt as _;

use crate::datastructures::cli_formating::title::Heading1;
use crate::tools::stats::year_in_music::YimReport;

static MONTHS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ]
});

impl YimReport {
    pub async fn monthly_recap_page(&self) -> String {
        let mut out = String::new();
        let stats = self.get_montly_stats().await;
        writeln!(out, "{}", Heading1("Monthly recap 📅")).unwrap();

        writeln!(out, "Here's your listen time per month:").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "{}", self.get_graph(stats)).unwrap();

        out
    }

    async fn get_montly_stats(&self) -> HashMap<u32, RecordingWithListensCollection> {
        let mut stats = HashMap::new();

        for i in 1..13 {
            let month_start = self.year_start.with_month(i).unwrap();
            let month_end = month_start.checked_add_months(Months::new(1)).unwrap();

            let listens = self
                .current
                .clone_no_stats()
                .filter_listening_date(month_start.into(), month_end.into());

            stats.insert(i, listens.recording_stats().await.unwrap().to_owned());
        }

        stats
    }

    fn get_graph(&self, stats: HashMap<u32, RecordingWithListensCollection>) -> String {
        let mut bars = Vec::with_capacity(12);

        for i in 1..13 {
            let recs = stats.get(&i).cloned().unwrap_or_default();
            let value = recs.get_time_listened().unwrap_or_default();
            let val_label = value.floor_to_minute().to_humantime().unwrap().to_string();

            bars.push(
                Data::builder()
                    .label(*MONTHS.get(i as usize - 1).unwrap())
                    .value(value.num_seconds())
                    .value_display(val_label)
                    .build(),
            );
        }

        BarGraph::builder()
            .width(50)
            .bar_color(Color(18, 198, 121))
            .build()
            .format_data(&bars)
    }
}
