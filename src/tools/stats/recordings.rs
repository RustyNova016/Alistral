use core::cmp::Reverse;

use itertools::Itertools;

use crate::datastructures::entity_with_listens::recording_with_listens::RecordingWithListens;
use crate::datastructures::listen_collection::ListenCollection;
use crate::utils::cli::display::RecordingExt;
use crate::utils::cli_paging::CLIPager;
use crate::utils::extensions::chrono_ext::DurationExt;

pub async fn stats_recording(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.len()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        println!(
            "[{}] {}",
            group.len(),
            group
                .recording()
                .pretty_format_with_credits(conn, true)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}

pub async fn stats_recording_time(conn: &mut sqlx::SqliteConnection, listens: ListenCollection) {
    let mut groups = RecordingWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();
    groups.sort_by_key(|a| Reverse(a.get_time_listened()));

    let mut pager = CLIPager::new(10);

    for group in groups {
        println!(
            "[{}] {}",
            group
                .get_time_listened()
                .map(|dur| dur.format_hh_mm())
                .unwrap_or_else(|| "??".to_string()),
            group
                .recording()
                .pretty_format_with_credits(conn, true)
                .await
                .expect("Error getting formated recording name"),
        );

        if !pager.inc() {
            break;
        }
    }
}
