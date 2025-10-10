use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike as _;
use chrono::Utc;
use itertools::Itertools;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync as _;

use crate::tools::daily::DailyCommand;
use crate::utils::constants::LISTENBRAINZ_FMT;

impl DailyCommand {
    pub async fn print_recording_birthdays(stats: &ListenStatisticsData, today: DateTime<Utc>) {
        let mut recordings = Self::get_recording_birthdays(stats, today).await;

        if recordings.is_empty() {
            return;
        }

        println!("{}", " Today in history 🎂 ".on_green().black().bold());
        recordings.sort_by_cached_key(|r| Reverse(r.listen_count()));

        for rec in recordings {
            println!(
                "   - {} ({}, {} Listens)",
                rec.recording()
                    .format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                rec.recording().first_release_date().unwrap().year(),
                rec.listen_count()
            );
        }

        println!();
    }

    async fn get_recording_birthdays(
        stats: &ListenStatisticsData,
        today: DateTime<Utc>,
    ) -> Vec<RecordingWithListens> {
        let stats = stats.recording_stats().await.unwrap();

        stats
            .iter()
            .filter(|rec| {
                rec.entity().first_release_date().is_some_and(|release| {
                    release.day() == today.day() && release.month() == today.month()
                })
            })
            .cloned()
            .collect_vec()
    }
}
