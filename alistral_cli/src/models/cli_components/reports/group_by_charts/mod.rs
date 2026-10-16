use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use charchart::bar_graph::BarGraph;
use charchart::bar_graph::colors::Color;
use charchart::bar_graph::data::Data;
use itertools::Itertools;
use musicbrainz_db_lite::models::listenbrainz::listen::Listen;

use crate::models::cli_components::formaters::human_duration::HumanDurationFormat;

pub mod days;
pub mod month;

pub fn group_by_report<F>(
    labels: &[(u32, &str)],
    mut group_by_function: F,
    current_listens: impl ListenCollectionReadable,
    previous_listens: impl ListenCollectionReadable,
) -> String
where
    F: FnMut(&Listen) -> u32,
{
    let mut current = current_listens
        .iter_unique_listens()
        .into_group_map_by(|listen| group_by_function(*listen));
    let mut previous = previous_listens
        .iter_unique_listens()
        .into_group_map_by(|listen| group_by_function(*listen));

    let mut bars = Vec::with_capacity(labels.len());
    for (num, label) in labels {
        let current_val = current.remove(num).unwrap_or_default();
        let current_duration = current_val.total_duration();
        let current_dur_string = HumanDurationFormat(current_duration).to_string();

        let previous_val = previous.remove(num).unwrap_or_default();
        let previous_duration = previous_val.total_duration();
        let previous_dur_string = HumanDurationFormat(previous_duration).to_string();

        bars.push(
            Data::builder()
                .label(*label)
                .value(current_duration.num_seconds())
                .value_display(current_dur_string)
                .build(),
        );

        bars.push(
            Data::builder()
                .label(*label)
                .value(previous_duration.num_seconds())
                .value_display(previous_dur_string)
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
