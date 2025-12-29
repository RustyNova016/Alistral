use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;

use crate::models::datastructures::tops::printer::top_columns::TopColumnSort;
use crate::models::datastructures::tops::printer::top_columns::TopColumnType;
use crate::models::datastructures::tops::printer::top_row::TopRow;

#[derive(Debug, bon::Builder)]
pub struct TopTablePrinter {
    /// The list of columns to display
    columns: Vec<TopColumnType>,

    sorted_column: Option<TopColumnType>,

    #[builder(default = TopColumnSort::Desc)]
    sort_order: TopColumnSort,
}

impl TopTablePrinter {
    pub async fn add_vec(&self, table: &mut Table, rows: Vec<TopRow>) {
        for row in rows {
            self.add_row(table, &row).await;
        }
    }

    async fn add_row(&self, table: &mut Table, row: &TopRow) {
        let mut new_row = Vec::new();

        for col in &self.columns {
            match col {
                TopColumnType::Rank => new_row.push(row.rank_col()),
                TopColumnType::Title => new_row.push(row.element_col().await),
                TopColumnType::ListenCount => new_row.push(row.listen_col(5, 5)),
                TopColumnType::ListenDuration => new_row.push(row.score_col()),
            }
        }

        table.add_row(new_row);
    }

    pub fn create_table(&self) -> Table {
        let mut table_model = Table::new();
        let mut columns = Vec::new();

        let sort_char = match self.sort_order {
            TopColumnSort::Asc => '▲',
            TopColumnSort::Desc => '▼',
        };

        for col in &self.columns {
            if self.sorted_column.is_some_and(|sort_col| &sort_col == col) {
                columns.push(format!("{sort_char} {col}"));
            } else {
                columns.push(col.to_string());
            }
        }

        table_model
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(columns);

        // // Set the default alignment for the score column to right
        // let column = table_model
        //     .column_mut(1)
        //     .expect("Our table has at least 2 columns");
        // column.set_cell_alignment(CellAlignment::Right);

        table_model
    }

    pub async fn format_n_rows(&self, mut rows: Vec<TopRow>, count: usize) -> String {
        let mut table = self.create_table();

        let _ = rows.split_off(count);

        self.add_vec(&mut table, rows).await;

        table.to_string()
    }
}
