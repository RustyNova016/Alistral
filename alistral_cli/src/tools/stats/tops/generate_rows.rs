use core::cmp::Reverse;

use alistral_core::datastructures::entity_with_listens::recording::RecordingWithListens;
use alistral_core::datastructures::entity_with_listens::recording::collection::RecordingWithListensCollection;
use alistral_core::datastructures::entity_with_listens::traits::IterRecordingWithListens;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use alistral_core::models::listen_statistics_data::ListenStatisticsData;
use comfy_table::CellAlignment;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use itertools::Itertools;
use musicbrainz_db_lite::HasRowID;
use musicbrainz_db_lite::Recording;
use sequelles::datastructures::ranking::Ranking;
use tuillez::formatter::FormatWithAsync;

use crate::ALISTRAL_CLIENT;
use crate::interface::tops::top_row::TopRow;
use crate::tools::stats::tops::StatsTopCommand;
use crate::utils::cli::await_next;
use crate::utils::constants::LISTENBRAINZ_FMT;
use crate::utils::user_inputs::UserInputParser;

impl StatsTopCommand {
    pub async fn get_current_recording_rankings(&self) {
        let (cur_stats, before_stats) = self.get_stats().await;

        let cur_stats = cur_stats.recording_stats().await.unwrap();
    }

    pub async fn generate_recording_rows(&self) -> Vec<TopRow<Recording>> {
        let (cur_stats, before_stats) = self.get_stats().await;
        let before_stats = before_stats.unwrap();

        let cur_stats = cur_stats.recording_stats().await.unwrap();
        let before_stats = before_stats.recording_stats().await.unwrap();

        let all_recordings = cur_stats
            .iter_entities()
            .chain(before_stats.iter_entities())
            .unique_by(|ent| ent.rowid())
            .cloned()
            .collect_vec();

        let cur_stats = create_ranking(&all_recordings, cur_stats.to_owned());
        let before_stats = create_ranking(&all_recordings, before_stats.to_owned());

        let cur_stats = cur_stats.get_ranks(|rec| Reverse(rec.listen_count()));
        let before_stats = before_stats.get_ranks(|rec| Reverse(rec.listen_count()));

        let mut rows = Vec::new();
        for (rank, recording) in cur_stats {
            let before = before_stats
                .iter()
                .find(|(_, rec)| rec.entity().rowid() == recording.entity().rowid());

            rows.push(TopRow {
                ranking: rank + 1,
                score: recording.listen_count().to_string(),
                element: recording.entity().clone(),

                previous_ranking: before.cloned().map(|(rank, _)| rank + 1),
                previous_score: before.map(|(_, rec)| rec.listen_count().to_string()),
            })
        }

        rows
    }

    pub async fn print_rows(rows: Vec<TopRow<Recording>>) {
        let mut table_model = Table::new();
        table_model
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(vec!["Rank", "Score", "Title"]);

        // Set the default alignment for the score column to right
        let column = table_model
            .column_mut(1)
            .expect("Our table has three columns");
        column.set_cell_alignment(CellAlignment::Right);

        let mut i: i8 = 0;
        let mut table = table_model.clone();
        for row in rows {
            table.add_row(vec![
                row.rank_col(),
                row.score_col(),
                row.element
                    .format_with_async(&LISTENBRAINZ_FMT)
                    .await
                    .unwrap(),
            ]);

            if i > 13 {
                println!("{table}");
                await_next();
                i = 0;
                table = table_model.clone()
            } else {
                i += 1
            }
        }
    }

    pub async fn print_recording_stats(&self) {
        let stats = self.generate_recording_rows().await;
        Self::print_rows(stats).await;
    }
}

fn create_ranking(
    ents: &Vec<Recording>,
    stats: RecordingWithListensCollection,
) -> Ranking<RecordingWithListens> {
    let mut ranking = Ranking::new();

    for ent in ents {
        match stats.get_by_id(ent.rowid()) {
            Some(stat) => ranking.push(stat.to_owned()),
            None => ranking.push(RecordingWithListens::new(ent.clone(), Default::default())),
        }
    }

    ranking
}
