use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use sequelles::datastructures::ranking::Ranking;

use crate::models::cli_components::ask_continue::cli_ask_continue;
use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::maybe_reversed::MaybeReversed;

pub struct TopTable<R, S> {
    rows: Vec<R>,

    sort: S,
    reversed: bool,
}

impl<R, S> TopTable<R, S>
where
    R: TableRow,
    S: TableSort<R>,
{
    pub fn new(rows: Vec<R>, sort: S, reversed: bool) -> Self {
        Self {
            rows,
            sort,
            reversed,
        }
    }

    pub async fn format(&self, nb_rows: usize, offset: usize) -> String {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(R::get_table_header());

        // Rank the rows
        let mut ranking = Ranking::new();
        for row in &self.rows {
            ranking.push(row);
        }
        let rankings = ranking.get_ranks(|row| {
            let score = self.sort.get_row_score(row);

            if self.reversed {
                MaybeReversed::Reversed(score)
            } else {
                MaybeReversed::Normal(score)
            }
        });

        for (num, (rank, row)) in rankings.into_iter().enumerate() {
            if num < offset || nb_rows + offset <= num {
                continue;
            }

            table.add_row(row.format(rank + 1).await);
        }

        table.to_string()
    }

    /// Print all the table with paging
    pub async fn print_paged(&self, nb_rows: usize) {
        let mut offset = 0;

        while offset < self.rows.len() {
            println!("{}", self.format(nb_rows, offset).await);

            if !cli_ask_continue() {
                return;
            }

            offset += nb_rows;
        }
    }
}

// impl<Ent, O> OrderWith<O> for TopTable<Ent>
// where
//     Vec<Ent>: OrderWith<O>,
// {
//     fn order_with_mut(&mut self, orderer: O) -> &mut Self {
//         self.rows.order_with_mut(orderer);
//         self
//     }
// }
