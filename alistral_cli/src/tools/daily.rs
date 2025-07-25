use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::artist::collection::ArtistWithRecordingsCollection;
use alistral_core::datastructures::listen_collection::ListenCollection;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Utc;
use itertools::Itertools;
use musicbrainz_db_lite::models::musicbrainz::release_group::ReleaseGroup;
use tracing::instrument;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::api::listenbrainz::fresh_releases::FreshReleaseRelease;
use crate::api::listenbrainz::fresh_releases::FreshReleaseRequest;
use crate::database::interfaces::statistics_data::artist_strategy;
use crate::database::interfaces::statistics_data::recording_stats;
use crate::database::musicbrainz::anniversaries::get_recordings_aniversaries;
use crate::models::config::Config;
use crate::utils::constants::LISTENBRAINZ_FMT;

#[instrument(skip(conn))]
pub async fn daily_report(conn: &mut sqlx::SqliteConnection, username: &str) {
    let recordings = recording_stats(&ALISTRAL_CLIENT, username.to_string())
        .await
        .expect("Couldn't fetch the listened recordings");

    // release days
    let today = Utc::now();
    //let today = Utc.timestamp_opt(1728508067, 0).unwrap();
    let release_day_recordings = get_recordings_aniversaries(conn, &today)
        .await
        .expect("Couldn't get the recording anniversaries");

    let mut anniversary_recordings = release_day_recordings
        .iter()
        .filter_map(|rec| recordings.get_by_id(rec.id))
        .collect_vec();

    let fresh_releases = get_fresh_releases(recordings.clone().into(), today).await;

    if !anniversary_recordings.is_empty() {
        println!("{}", " Today in history 🎂 ".on_green().black().bold());

        anniversary_recordings.sort_by_cached_key(|r| Reverse(r.listen_count()));

        for rec in anniversary_recordings {
            println!(
                "   - {} ({}, {} Listens)",
                rec.recording()
                    .format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                Utc.timestamp_opt(rec.recording().first_release_date.unwrap(), 0)
                    .unwrap()
                    .year(),
                rec.listen_count()
            );
        }

        println!();
    }

    let mut first_discoveries = recordings
        .iter()
        .filter(|rec| {
            rec.oldest_listen_date()
                .is_some_and(|date| date.day() == today.day() && date.month() == today.month())
        })
        .collect_vec();

    if !first_discoveries.is_empty() {
        println!(
            "{}",
            " You discovered those on this day 🔎 "
                .on_green()
                .black()
                .bold()
        );

        first_discoveries.sort_by_cached_key(|r| Reverse(r.listen_count()));

        for rec in first_discoveries {
            println!(
                "   - {} ({}, {} Listens)",
                rec.recording()
                    .format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                rec.oldest_listen_date()
                    .expect("There should be at least a listen")
                    .format("%d/%m/%Y"),
                rec.listen_count()
            );
        }

        println!();
    }

    if !fresh_releases.is_empty() {
        println!(
            "{}",
            " Fresh releases of the past 7 days 🌱 "
                .on_green()
                .black()
                .bold()
        );

        for fresh_release in fresh_releases {
            let rg = ReleaseGroup::get_or_fetch(
                conn,
                &ALISTRAL_CLIENT.musicbrainz_db,
                &fresh_release.release_group_mbid,
            )
            .await
            .expect("Couldn't retrieve release group data")
            .expect("Couldn't find the release group");

            println!(
                "   - {} {}",
                rg.format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                format!(
                    "({})",
                    Utc.timestamp_opt(rg.first_release_date.unwrap(), 0)
                        .unwrap()
                        //TODO: #407
                        .format("%d/%m/%Y")
                )
                .truecolor(175, 175, 175)
            );
        }
    }
}

#[instrument(skip_all)]
async fn get_fresh_releases(
    listens: ListenCollection,
    today: DateTime<Utc>,
) -> Vec<FreshReleaseRelease> {
    let fresh_releases = FreshReleaseRequest::builder()
        .days(7)
        .future(false)
        .past(true)
        .release_date(today)
        .build()
        .fetch()
        .await
        .expect("Couldn't get fresh listens")
        .payload
        .releases;

    //TODO: #529 Daily: Prevent recompiling recording with listens
    let artists = ArtistWithRecordingsCollection::from_listencollection(
        listens,
        &artist_strategy(&ALISTRAL_CLIENT),
    )
    .await
    .expect("Couldn't get the listened artists");

    let min_avg = Config::load_or_panic()
        .read_or_panic()
        .get_artist_listened_to_threshold();
    let listened_artist_ids = artists
        .into_iter()
        .filter(|artist| artist.average_listen_per_entity_listened() >= min_avg)
        .map(|artist| artist.entity().mbid.clone())
        .collect_vec();

    let mut out = fresh_releases
        .into_iter()
        .filter(|release| {
            release
                .artist_mbids
                .iter()
                .any(|mbid| listened_artist_ids.contains(mbid))
        })
        .collect_vec();

    out.sort_by_cached_key(|r| {
        let date = NaiveDate::parse_from_str(&r.release_date, "%Y-%m-%d")
            .expect("Invalid date returned from Listenbrainz");
        Reverse(date)
    });

    out
}

// #[cfg(test)]
// mod tests {
//     use crate::database::get_conn;
//     use crate::tools::daily::daily_report;
//     #[tokio::test]
//     #[serial_test::serial]
//     async fn daily_report_test() {
//         let mut conn = get_conn().await;
//         daily_report(&mut conn, "RustyNova").await;
//     }
// }
