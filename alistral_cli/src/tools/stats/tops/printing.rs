use comfy_table::CellAlignment;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use musicbrainz_db_lite::models::musicbrainz::MusicbrainzFormater;
use tuillez::formatter::FormatWithAsync;

use crate::interface::tops::top_row::TopRow;
use crate::tools::stats::tops::generator::TopGenerator;
use crate::utils::cli::await_next;
use crate::utils::constants::LISTENBRAINZ_FMT;

impl TopGenerator {
    /// Print the rows into tables
    pub async fn print_rows<T>(rows: Vec<TopRow<T>>)
    where
        T: FormatWithAsync<MusicbrainzFormater>,
        <T as FormatWithAsync<MusicbrainzFormater>>::Error: std::fmt::Debug,
    {
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
}
