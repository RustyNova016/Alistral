use alistral_core::datastructures::listen_collection::ListenCollection;
use charchart::bar_graph::BarGraph;
use charchart::bar_graph::colors::Color;
use charchart::bar_graph::data::Data;
use itertools::Itertools as _;

pub fn listen_count_per_year_graph(listens: ListenCollection) -> String {
    let map = listens.group_by_listen_year();
    let mut map_vec = map.into_iter().collect_vec();
    map_vec.sort_by_key(|elem| elem.0);

    let mut bars = Vec::new();
    for (year, data) in map_vec {
        bars.push(
            Data::builder()
                .label(year.to_string())
                .value(data.len())
                .value_display(data.len().to_string())
                .build(),
        );
    }

    BarGraph::builder()
        .width(50)
        .bar_color(Color(18, 198, 121))
        .build()
        .format_data(&bars)
}
