use core::cmp::Reverse;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::traits::ListenCollWithTime;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable as _;
use itertools::Itertools;
use sequelles::datastructures::ranking::Ranking;
use tuillez::OwoColorize;

use crate::ALISTRAL_CLIENT;
use crate::datastructures::cli_formating::title::Heading1;
use crate::datastructures::mappers::stats_by_release_year::StatsByReleaseYear;
use crate::models::datastructures::tops::printer::TopPrinter;
use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::models::datastructures::tops::top_score::TopScore;
use crate::tools::stats::year_in_music::YimReport;

impl YimReport {
    pub async fn new_release_page(&self) -> String {
        let mut out = String::new();
        let stats = self.get_stats().await;
        writeln!(out, "{}", Heading1("New releases of the year 🌱")).unwrap();

        writeln!(out, "{}", self.new_releases().await).unwrap();
        writeln!(out).unwrap();
        writeln!(out, "Here's the top 10 tracks:").unwrap();
        writeln!(out, "{}", Self::top(stats).await).unwrap();

        out
    }

    async fn get_stats(&self) -> Vec<RecordingWithListens> {
        let recordings = self.current.recording_stats().await.unwrap();

        let mut listen_by_dates = StatsByReleaseYear::default();
        listen_by_dates
            .insert_recording_stats_collection(&ALISTRAL_CLIENT.core, recordings.to_owned())
            .await
            .unwrap();

        listen_by_dates
            .mapping
            .as_mut_hash_map()
            .retain(|_, (date, _)| date.is_some_and(|date| date == self.year as i64));

        listen_by_dates
            .mapping
            .into_iter()
            .flat_map(|(_, row)| row)
            .collect_vec()
    }

    async fn new_releases(&self) -> String {
        let recordings = self.current.recording_stats().await.unwrap();

        let mut listen_by_dates = StatsByReleaseYear::default();
        listen_by_dates
            .insert_recording_stats_collection(&ALISTRAL_CLIENT.core, recordings.to_owned())
            .await
            .unwrap();

        listen_by_dates
            .mapping
            .as_mut_hash_map()
            .retain(|_, (date, _)| date.is_some_and(|date| date == self.year as i64));

        let track_count = listen_by_dates
            .mapping
            .as_hash_map()
            .values()
            .flat_map(|row| &row.1)
            .count();
        let listen_count = listen_by_dates
            .mapping
            .as_hash_map()
            .values()
            .flat_map(|row| &row.1)
            .flat_map(|rec| rec.iter_listens())
            .count();

        format!(
            "You listened to {} tracks released this year, which make up a total of {} listens",
            track_count.green(),
            listen_count.green()
        )
    }

    async fn top(stats: Vec<RecordingWithListens>) -> String {
        let rankings = Ranking::from(stats);
        let rankings = rankings.get_ranks(|rec| Reverse(rec.get_time_listened().unwrap_or_default()));

        let rows = rankings
            .into_iter()
            .map(|(rank, rec)| TopRow {
                ranking: rank,
                score: TopScore::TimeDelta(rec.get_time_listened().unwrap_or_default()),
                element: Box::new(rec.recording().clone()),
                previous_ranking: None,
                previous_score: None,
            })
            .collect_vec();

        TopPrinter::format_n_rows(rows, 10).await
    }
}
