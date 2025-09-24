mod database_atributes;
mod derives;
extern crate proc_macro;

use derives::main_entity::derive_main_entity_impl;
use proc_macro::TokenStream;

#[proc_macro_derive(MainEntity, attributes(database))]
pub fn derive_main_entity(item: TokenStream) -> TokenStream {
    derive_main_entity_impl(item)
}
