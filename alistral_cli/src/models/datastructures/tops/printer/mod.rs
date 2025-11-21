use comfy_table::CellAlignment;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;

use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::utils::cli::await_next;

pub mod top_row;
pub struct TopPrinter;

impl TopPrinter {
    pub fn get_table() -> Table {
        let mut table_model = Table::new();
        table_model
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(vec!["Rank", "Score", "Title"]);

        // Set the default alignment for the score column to right
        let column = table_model
            .column_mut(1)
            .expect("Our table has three columns");
        column.set_cell_alignment(CellAlignment::Right);

        table_model
    }

    pub async fn print_rows(rows: Vec<TopRow>) {
        let table_model = Self::get_table();

        let mut i: i8 = 0;
        let mut table = table_model.clone();
        for row in rows {
            table.add_row(vec![
                row.rank_col(),
                row.score_col(),
                row.element_col().await,
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

    pub async fn format_n_rows(rows: Vec<TopRow>, count: usize) -> String {
        let mut table = Self::get_table();

        for i in 0..count {
            if let Some(row) = rows.get(i) {
                table.add_row(vec![
                    row.rank_col(),
                    row.score_col(),
                    row.element_col().await,
                ]);
            }
        }

        table.to_string()
    }
}
