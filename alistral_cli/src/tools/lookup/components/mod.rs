pub mod disp_duration;
pub mod previous_tf_comparison;
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
pub(super) struct LookupLine<Ent, Lis, Fdata, Data, Form>
where
    Ent: RowId,
    Lis: ListenCollectionReadable + ExtractTimeframe,
    Fdata: Fn(&EntityWithListens<Ent, Lis>) -> Data,
    Data: Display,
    Form: Fn(&Data) -> String,
{
    description: String,
    data: ListenTimeframe<EntityWithListens<Ent, Lis>>,

    get_data: Fdata,
    #[builder(default = false)]
    lower_is_better: bool,

    // Formaters

    /// Format the value returned by [`Self::get_data`] into a nice string to display
    value_formater: Option<Form>,
}

impl<Ent, Lis, Fdata, Data, Form> LookupLine<Ent, Lis, Fdata, Data, Form>
where
    Ent: RowId,
    Lis: ListenCollectionReadable + ExtractTimeframe,
    Fdata: Fn(&EntityWithListens<Ent, Lis>) -> Data,
    Data: Display + PartialOrd,
    Form: Fn(&Data) -> String,
{
    pub fn get_arrow(&self, current: &Data, previous: &Data) -> String {
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
        let mut out = format!("{}: {} ", self.description, self.format_value(&current_value));

        if let Some(prev_data) = &self.data.previous_opt() {
            let prev_value = (self.get_data)(&prev_data);
            write!(
                out,
                "[{} - {}]",
                self.get_arrow(&current_value, &prev_value),
                self.format_value(&prev_value)
            )
            .unwrap()
        }

        out
    }

    pub fn format_value(&self, value: &Data) -> String {
        match &self.value_formater {
            Some(v) => (v)(value),
            None => value.to_string()
        }
    }
}
