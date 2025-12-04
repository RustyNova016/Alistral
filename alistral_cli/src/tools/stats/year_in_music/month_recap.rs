use std::fmt::Write;
use std::sync::LazyLock;

use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use charchart::bar_graph::BarGraph;
use charchart::bar_graph::colors::Color;
use charchart::bar_graph::data::Data;
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
        writeln!(out, "{}", Heading1("Monthly recap 📅")).unwrap();

        writeln!(out, "Here's your listen time per month:").unwrap();
        writeln!(out).unwrap();
        writeln!(out, "{}", self.get_graph().await).unwrap();

        out
    }

    async fn get_graph(&self) -> String {
        let mut bars = Vec::with_capacity(12);

        let current_map = self.data.listens_per_month_current().await;
        let previous_map = self.data.listens_per_month_previous().await;

        for month in 1..13 {
            let current_time_list = if let Some(data) = current_map.get(&month) {
                data.recording_stats()
                    .await
                    .map(|stats| stats.to_owned())
                    .unwrap_or_default()
                    .to_owned()
                    .get_time_listened()
                    .unwrap_or_default()
            } else {
                Default::default()
            };

            let current_var_label = current_time_list
                .floor_to_minute()
                .to_humantime()
                .unwrap()
                .to_string();

            bars.push(
                Data::builder()
                    .label(*MONTHS.get(month as usize - 1).unwrap())
                    .value(current_time_list.num_seconds())
                    .value_display(current_var_label)
                    .build(),
            );

            let prev_time_list = if let Some(data) = previous_map.get(&month) {
                data.recording_stats()
                    .await
                    .map(|stats| stats.to_owned())
                    .unwrap_or_default()
                    .to_owned()
                    .get_time_listened()
                    .unwrap_or_default()
            } else {
                Default::default()
            };

            let prev_var_label = prev_time_list
                .floor_to_minute()
                .to_humantime()
                .unwrap()
                .to_string();

            bars.push(
                Data::builder()
                    .label(*MONTHS.get(month as usize - 1).unwrap())
                    .value(prev_time_list.num_seconds())
                    .value_display(prev_var_label)
                    .bar_color(Color(18, 121, 198))
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
