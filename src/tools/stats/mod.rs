use crate::database::get_db_client;
use crate::database::listenbrainz::listens::{ListenFetchQuery, ListenFetchQueryReturn};
use crate::models::cli::common::{GroupByTarget, SortSorterBy};
use crate::models::data::listenbrainz::user_listens::UserListens;
use crate::tools::stats::release_groups::stats_release_groups;
use crate::utils::println_cli;

use self::work::stats_works;

mod artists;
mod recordings;
mod release_groups;
mod releases;
pub mod work;

pub async fn stats_command(username: &str, target: GroupByTarget, sort_by: SortSorterBy) {
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(get_db_client().await)
        .await
        .expect("Couldn't fetch the new listens");

    //recordings::stats_recording(listens).await;
    releases::stats_releases(listens).await;
    panic!();

    // Get the listens
    let user_listens = UserListens::get_user_with_refresh(username)
        .await
        .expect("Couldn't fetch the new listens");

    println_cli(format!(
        "Total number of listens: {}",
        user_listens.get_listens().len()
    ));

    let stats = user_listens
        .get_listens()
        .get_statistics_of(target)
        .await
        .expect("Couldn't sort the listens");

    match target {
        GroupByTarget::Recording => {
            recordings::stats_recording(listens).await;
        }
        GroupByTarget::Artist => {
            artists::stats_artist(stats, sort_by).await;
        }
        GroupByTarget::Release => {
            releases::stats_releases(listens).await;
        }
        GroupByTarget::ReleaseGroup => {
            stats_release_groups(stats, sort_by).await;
        }
        GroupByTarget::Work => {
            stats_works(stats, sort_by).await;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::utils::println_cli_info;

    use super::*;

    #[tokio::test]
    #[serial_test::serial]
    async fn stats_command_recordings() {
        //let mut clog = colog::default_builder();
        //clog.filter(None, log::LevelFilter::Trace);
        //clog.init();
        
        println_cli_info("--- Starting test ---");
        stats_command("backhdlp", GroupByTarget::Recording, SortSorterBy::Count).await;


    }
}
