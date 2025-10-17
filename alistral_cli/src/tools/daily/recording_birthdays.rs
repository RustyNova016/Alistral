use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use chrono::DateTime;
use chrono::Datelike as _;
use chrono::Local;
use tracing::instrument;
use tuillez::OwoColorize as _;
use tuillez::formatter::FormatWithAsync as _;
use tuillez::pg_counted;
use tuillez::pg_inc;
use tuillez::pg_spinner;

use crate::ALISTRAL_CLIENT;
use crate::tools::daily::DailyCommand;
use crate::utils::constants::LISTENBRAINZ_FMT;

impl DailyCommand {
    pub async fn print_recording_birthdays(stats: &ListenStatisticsData, today: DateTime<Local>) {
        let mut recordings = Self::get_recording_birthdays(stats, today).await;

        if recordings.is_empty() {
            return;
        }

        println!("{}", " Today in history 🎂 ".on_green().black().bold());
        recordings.sort_by_cached_key(|r| Reverse(r.listen_count()));

        for rec in recordings {
            let date = rec
                .entity()
                .first_release_date_or_fetch(ALISTRAL_CLIENT.musicbrainz_db.clone())
                .await
                .unwrap()
                .unwrap();

            println!(
                "   - {} ({}, {} Listens)",
                rec.recording()
                    .format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .expect("Couldn't get artist credits"),
                date.year(),
                rec.listen_count()
            );
        }

        println!();
    }

    #[instrument(skip(stats), fields(indicatif.pb_show = tracing::field::Empty))]
    async fn get_recording_birthdays(
        stats: &ListenStatisticsData,
        today: DateTime<Local>,
    ) -> Vec<RecordingWithListens> {
        pg_spinner!("Generating `Recording birthdays` Releases` report");

        let stats = stats.recording_stats().await.unwrap();

        pg_counted!(
            stats.len(),
            "Generating `Recording birthdays` Releases` report"
        );

        let mut recs = Vec::new();
        for rec in stats.iter() {
            pg_inc!();
            let Some(release) = rec
                .entity()
                .first_release_date_or_fetch(ALISTRAL_CLIENT.musicbrainz_db.clone())
                .await
                .unwrap()
            else {
                continue;
            };

            if release.day() == today.day() && release.month() == today.month() {
                recs.push(rec.clone())
            }
        }

        recs
    }
}
