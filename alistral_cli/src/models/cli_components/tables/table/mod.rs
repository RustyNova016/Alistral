pub mod from_entity_listens;
use std::collections::HashMap;

use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use itertools::Itertools;
use sequelles::datastructures::ranking::Ranking;
use tracing::warn;

use crate::models::cli_components::ask_continue::cli_ask_continue;
use crate::models::cli_components::tables::TableSort;
use crate::models::cli_components::tables::cells::rank_cell::RankCell;
use crate::models::cli_components::tables::cells::top_cell::TopCell;
use crate::models::cli_components::tables::rows::TableRow;
use crate::models::maybe_reversed::MaybeReversed;
use crate::utils::extensions::sequelles::hashjoin::HashJoin;

pub struct TopTable<R, S> {
    rows: Vec<R>,

    sort: S,
    reversed: bool,
    show_prev: bool,
}

impl<R, S> TopTable<R, S>
where
    R: TableRow,
    S: TableSort<R>,
{
    pub fn new(rows: Vec<R>, sort: S, reversed: bool, show_prev: bool) -> Self {
        Self {
            rows,
            sort,
            reversed,
            show_prev,
        }
    }

    pub async fn format(&self, nb_rows: usize, offset: usize) -> String {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_header(R::get_table_header());

        let mut rows = HashMap::new();
        for row in self.rows.iter() {
            rows.insert(row.get_row_id(), row);
        }

        let rankings = self.get_rank_cells();

        let rows = rows.hash_join(&rankings);

        for (num, (row, ranking)) in rows
            .into_values()
            .filter_map(|(row, rank)| row.map(|row| (row, rank)))
            .sorted_by_cached_key(|(row, _)| self.get_row_ord(row))
            .enumerate()
        {
            if num < offset || nb_rows + offset <= num {
                continue;
            }

            let Some(ranking) = ranking else {
                warn!("A row is missing a ranking");
                continue;
            };

            table.add_row(row.format(ranking.to_owned(), self.show_prev).await);
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

    pub fn get_rank_cells(&self) -> HashMap<i64, RankCell> {
        let mut current = HashMap::new();
        for (rank, row) in self.get_current_rankings() {
            current.insert(row.get_row_id(), rank + 1);
        }

        let mut previous = HashMap::new();
        for (rank, row) in self.get_previous_rankings() {
            previous.insert(row.get_row_id(), rank + 1);
        }

        current
            .hash_join(&previous)
            .into_iter()
            .map(|(id, (current, previous))| {
                (
                    *id,
                    RankCell(TopCell::new(current.cloned(), previous.cloned())),
                )
            })
            .collect()
    }

    fn get_current_rankings(&self) -> Vec<(usize, &R)> {
        let mut ranking = Ranking::new();

        for row in &self.rows {
            ranking.push(row);
        }

        ranking.get_ranks(|row| {
            let score = self.sort.get_row_score(row);

            if self.reversed {
                MaybeReversed::Reversed(score)
            } else {
                MaybeReversed::Normal(score)
            }
        })
    }

    fn get_previous_rankings(&self) -> Vec<(usize, &R)> {
        let mut ranking = Ranking::new();

        for row in &self.rows {
            if self.sort.get_row_prev_score(row).is_none() {
                continue;
            }
            ranking.push(row);
        }

        ranking.get_ranks(|row| {
            let score = self.sort.get_row_prev_score(row).unwrap();

            if self.reversed {
                MaybeReversed::Reversed(score)
            } else {
                MaybeReversed::Normal(score)
            }
        })
    }

    pub fn get_row_ord(&self, row: &R) -> impl Ord {
        let score = self.sort.get_row_score(row);

        if self.reversed {
            MaybeReversed::Reversed(score)
        } else {
            MaybeReversed::Normal(score)
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
