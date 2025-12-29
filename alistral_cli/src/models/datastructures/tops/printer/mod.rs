use comfy_table::CellAlignment;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;

use crate::models::datastructures::tops::printer::top_row::TopRow;

#[cfg(feature = "stats")]
pub mod top_row;

pub struct TopPrinter;

impl TopPrinter {
    pub fn get_table() -> Table {
        let mut table_model = Table::new();
        table_model
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(vec!["Rank", "Score", "Listens", "Title"]);

        // Set the default alignment for the "score" column to right
        let score_column = table_model
            .column_mut(1)
            .expect("Couldn't get a reference to the score column");
        score_column.set_cell_alignment(CellAlignment::Right);

        // Set the default alignment for the "listens" column to right
        let listens_column = table_model
            .column_mut(2)
            .expect("Couldn't get a reference to the score column");
        listens_column.set_cell_alignment(CellAlignment::Right);

        table_model
    }

    pub async fn format_n_rows(rows: Vec<TopRow>, count: usize) -> String {
        let mut table = Self::get_table();

        for i in 0..count {
            if let Some(row) = rows.get(i) {
                table.add_row(vec![
                    row.rank_col(),
                    row.score_col(),
                    row.listen_count_col(),
                    row.element_col().await,
                ]);
            }
        }

        table.to_string()
    }
}
