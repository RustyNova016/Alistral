use alistral_core::database::fetching::listens::ListenFetchQuery;
use alistral_core::database::fetching::listens::ListenFetchQueryReturn;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::common::SortSorterBy;
use crate::models::cli::common::StatsTarget;

mod artists;
mod recordings;
mod release_groups;
mod releases;
mod work;

pub async fn stats_command(
    conn: &mut sqlx::SqliteConnection,
    username: &str,
    target: StatsTarget,
    _sort_by: SortSorterBy,
) {
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn, &ALISTRAL_CLIENT.core)
        .await
        .expect("Couldn't fetch the new listens");

    match target {
        StatsTarget::Recording => {
            recordings::stats_recording(conn, listens).await;
        }
        StatsTarget::RecordingPlaytime => {
            recordings::stats_recording_time(conn, listens).await;
        }
        StatsTarget::Artist => {
            artists::stats_artist(conn, listens).await;
        }
        StatsTarget::Release => {
            releases::stats_releases(conn, listens).await;
        }
        StatsTarget::ReleaseGroup => {
            release_groups::stats_release_groups(conn, listens).await;
        }
        StatsTarget::Work => {
            work::stats_works(conn, listens).await;
        }
        StatsTarget::WorkRecursive => {
            work::stats_works_recursive(conn, listens).await;
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::database::get_conn;
//     use crate::models::cli::common::StatsTarget;
//     use crate::tools::stats::stats_command;
//     use crate::tools::stats::SortSorterBy;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn stats_command_recordings() {
//          let mut conn = get_conn().await;
//         stats_command(&mut conn, "RustyNova", StatsTarget::WorkRecursive, SortSorterBy::Count).await;
//     }
// }
