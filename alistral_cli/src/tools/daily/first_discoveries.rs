use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike as _;
use chrono::Local;
use itertools::Itertools;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync as _;

use crate::tools::daily::DailyCommand;
use crate::utils::constants::LISTENBRAINZ_FMT;

impl DailyCommand {
    pub async fn print_first_discoveries(stats: &ListenStatisticsData, today: DateTime<Local>) {
        let mut recordings = Self::get_first_discoveries(stats, today).await;

        if recordings.is_empty() {
            return;
        }

        println!(
            "{}",
            " You discovered those on this day 🔎 "
                .on_green()
                .black()
                .bold()
        );
        recordings.sort_by_cached_key(|r| Reverse(r.listen_count()));

        for rec in recordings {
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

    async fn get_first_discoveries(
        stats: &ListenStatisticsData,
        today: DateTime<Local>,
    ) -> Vec<RecordingWithListens> {
        let stats = stats.recording_stats().await.unwrap();

        stats
            .iter()
            .filter(|rec| {
                rec.listens().oldest_listen_date().is_some_and(|discovery| {
                    discovery.day() == today.day() && discovery.month() == today.month()
                })
            })
            .cloned()
            .collect_vec()
    }
}
