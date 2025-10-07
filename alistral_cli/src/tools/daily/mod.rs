use core::cmp::Reverse;

use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::Datelike;
use chrono::TimeZone;
use chrono::Utc;
use clap::Parser;
use itertools::Itertools;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::database::musicbrainz::anniversaries::get_recordings_aniversaries;
use crate::models::config::Config;
use crate::utils::constants::LISTENBRAINZ_FMT;

pub mod fresh_releases;

/// A combination of small statistics to run daily
#[derive(Parser, Debug, Clone)]
pub struct DailyCommand {
    /// Name of the user to provide a daily report
    username: Option<String>,
}

impl DailyCommand {
    pub async fn run(&self) {
        let stats = ListenStatisticsData::new_from_user_listens(
            ALISTRAL_CLIENT.core.clone(),
            Config::check_username(&self.username),
        )
        .await
        .expect("Couldn't fetch user listens");

        Self::old_report(
            &mut ALISTRAL_CLIENT.musicbrainz_db.get_conn().await.unwrap(),
            &stats,
        )
        .await;
        Self::print_fresh_releases(&stats, Utc::now()).await;
    }

    pub async fn old_report(conn: &mut sqlx::SqliteConnection, user_data: &ListenStatisticsData) {
        let recordings = user_data
            .recording_stats()
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
    }
}
