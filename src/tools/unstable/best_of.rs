use core::cmp::Reverse;

use alistral_core::cli::progress_bar::global_progress_bar::PG_FETCHING;
use chrono::Utc;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;

use crate::database::listenbrainz::listens::ListenFetchQuery;
use crate::database::listenbrainz::listens::ListenFetchQueryReturn;
use crate::datastructures::entity_with_listens::release_with_listens::ReleaseWithListens;
use crate::utils::extensions::chrono_ext::DurationExt;

pub async fn best_of_checker(conn: &mut sqlx::SqliteConnection, username: &str) {
    let listens = ListenFetchQuery::builder()
        //.fetch_recordings_redirects(true)
        .returns(ListenFetchQueryReturn::Mapped)
        .user(username.to_string())
        .build()
        .fetch(conn)
        .await
        .expect("Couldn't fetch the new listens");

    let releases = ReleaseWithListens::from_listencollection(conn, listens)
        .await
        .expect("Error while fetching recordings")
        .into_values()
        .collect_vec();

    let mut filtered = Vec::new();
    let progress_bar = PG_FETCHING.get_submitter(releases.len() as u64);
    for release in releases {
        progress_bar.inc(1);
        if release.release().date.is_none_or(|date| date <= 1704067200) {
            continue;
        }

        let label_info = release
            .release()
            .get_label_infos_or_fetch(conn)
            .await
            .expect("Couldn't get label info");

        if label_info.iter().any(|label| {
            let Some(label) = &label.label else {
                return false;
            };
            label == "e979fa51-f704-4a15-bd1f-1f9a1d4e4f0b" // Monstercat
                || label == "b10497d9-68c2-4f58-a9ae-8ba7b15d3e09" // uncaged
                || label == "708fae2b-558f-402f-84d4-fa9e2a2d83ad" // instinct
                || label == "2f7dd00a-031a-4021-9975-64ba4969dce2" // Silk
        }) {
            filtered.push(release);
        }
    }

    let recordings = filtered
        .into_iter()
        .flat_map(|r| r.listens().clone())
        .filter(|r| {
            r.recording()
                .first_release_date
                .is_some_and(|t| t >= 1704067200)
        })
        .unique_by(|r| r.recording().id)
        .collect_vec();

    // Top listened
    let mut recording_by_count = recordings.clone();
    recording_by_count.sort_by_cached_key(|release| Reverse(release.len()));
    let recording_by_count = recording_by_count.into_iter().enumerate().collect_vec();

    println!();
    println!("{}", "⨠ Top 20 listens".black().on_green());
    for i in 0..20 {
        let Some(recording) = recording_by_count.get(i) else {
            break;
        };

        println!(
            "[#{}] [{}] {}",
            i + 1,
            recording.1.len(),
            recording
                .1
                .recording()
                .format_with_credits(conn)
                .await
                .expect("Couldn't get recording's artist credits")
        );
    }

    let mut recording_by_rate = recordings.clone();
    recording_by_rate.sort_by_cached_key(|recording| {
        recording.average_duration_between_listens_and_date(Utc::now())
    });
    let recording_by_rate = recording_by_rate.into_iter().enumerate().collect_vec();

    println!();
    println!("{}", "⨠ Top 20 listen rates".black().on_green());
    for i in 0..20 {
        let Some(recording) = recording_by_rate.get(i) else {
            break;
        };

        println!(
            "[#{}] [{}] {}",
            i + 1,
            recording
                .1
                .average_duration_between_listens_and_date(Utc::now())
                .floor_to_minute()
                .to_humantime()
                .unwrap(),
            recording
                .1
                .recording()
                .format_with_credits(conn)
                .await
                .expect("Couldn't get recording's artist credits")
        );
    }

    let mut recording_by_average = recordings
        .clone()
        .into_iter()
        .map(|r| {
            let count = recording_by_count.iter().find_map(|c| {
                if c.1.recording().id == r.recording().id {
                    Some(c.0)
                } else {
                    None
                }
            });

            let rate = recording_by_rate.iter().find_map(|c| {
                if c.1.recording().id == r.recording().id {
                    Some(c.0)
                } else {
                    None
                }
            });

            match count {
                Some(count) => match rate {
                    Some(rate) => ((count + rate) / 2, r),
                    None => (count, r),
                },
                None => (
                    rate.expect("The recording should appear in rate or counts"),
                    r,
                ),
            }
        })
        .collect_vec();
    recording_by_average.sort_by_key(|(r, _)| *r);

    println!();
    println!("{}", "⨠ Top 20 average rank".black().on_green());
    for i in 0..20 {
        let Some(recording) = recording_by_average.get(i) else {
            break;
        };

        println!(
            "[#{}] [{}] {}",
            i + 1,
            recording.0,
            recording
                .1
                .recording()
                .format_with_credits(conn)
                .await
                .expect("Couldn't get recording's artist credits")
        );
    }

    println!();
}

// #[cfg(test)]
// mod tests {
//     use crate::utils::println_cli_info;

//     use super::*;

//     #[tokio::test]
//     #[serial_test::serial]
//     async fn unstable_bo24() {
//         //let mut clog = colog::default_builder();
//         //clog.filter(None, log::LevelFilter::Trace);
//         //clog.init();

//         println_cli_info("--- Starting test ---");
//         best_of_checker("RustyNova").await;
//     }
// }
