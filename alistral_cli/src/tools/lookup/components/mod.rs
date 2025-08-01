pub mod title;
use core::cmp::Ordering;
use core::fmt::Display;
use std::fmt::Write;

use alistral_core::datastructures::entity_with_listens::EntityWithListens;
use alistral_core::datastructures::entity_with_listens::collection::EntityWithListensCollection;
use alistral_core::datastructures::entity_with_listens::listen_timeframe::ListenTimeframe;
use alistral_core::datastructures::entity_with_listens::listen_timeframe::extract_timeframe::ExtractTimeframe;
use alistral_core::datastructures::listen_collection::traits::ListenCollectionReadable;
use musicbrainz_db_lite::RowId;
use tuillez::OwoColorize;
use tuillez::formatter::FormatWithAsync;

#[derive(bon::Builder, Clone, Debug)]
#[builder(derive(Clone, Debug))]
pub(super) struct LookupLine<E, L, F, T>
where
    E: RowId,
    L: ListenCollectionReadable + ExtractTimeframe,
    F: Fn(&EntityWithListens<E, L>) -> T,
    T: Display,
{
    description: String,
    data: ListenTimeframe<EntityWithListens<E, L>>,

    get_data: F,
    #[builder(default = false)]
    lower_is_better: bool,
}

impl<E, L, F, T> LookupLine<E, L, F, T>
where
    E: RowId,
    L: ListenCollectionReadable + ExtractTimeframe,
    F: Fn(&EntityWithListens<E, L>) -> T,
    T: Display + PartialOrd,
{
    pub fn get_arrow(&self, current: &T, previous: &T) -> String {
        match previous.partial_cmp(current) {
            None => "-".bright_black().to_string(),
            Some(Ordering::Equal) => "-".bright_black().to_string(),
            Some(Ordering::Less) if self.lower_is_better => "▲".red().to_string(),
            Some(Ordering::Less) => "▲".green().to_string(),
            Some(Ordering::Greater) if self.lower_is_better => "▼".green().to_string(),
            Some(Ordering::Greater) => "▼".red().to_string(),
        }
    }

    pub async fn to_string(&self) -> String {
        let current_value = (self.get_data)(&self.data.current());
        let mut out = format!("{}: {} ", self.description, current_value);

        if let Some(prev_data) = &self.data.previous_opt() {
            let prev_value = (self.get_data)(&prev_data);
            write!(
                out,
                "[{} - {}]",
                self.get_arrow(&current_value, &prev_value),
                prev_value
            )
            .unwrap()
        }

        out
    }
}
