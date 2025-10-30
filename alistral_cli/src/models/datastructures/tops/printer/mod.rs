use comfy_table::CellAlignment;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;


use crate::models::datastructures::tops::printer::top_row::TopRow;
use crate::utils::cli::await_next;

pub mod top_row;
pub struct TopPrinter;

impl TopPrinter {
    pub async fn print_rows(rows: Vec<TopRow>)

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
}

