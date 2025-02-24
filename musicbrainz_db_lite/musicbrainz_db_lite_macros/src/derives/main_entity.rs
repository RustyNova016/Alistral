use crate::derives::main_entity::selects::impl_selects;
use crate::derives::methods::update_date::impl_update_date;
use crate::{database_atributes::DatabaseAtribute, derives::methods::selects};
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

pub fn derive_main_entity_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let args = match DatabaseAtribute::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let struct_identifier = &input.ident;

    match &input.data {
        Data::Struct(syn::DataStruct { .. }) => {
            let full_update = impl_update_date(struct_identifier, &args.table, &args.primary_key);
            let selects = impl_selects(struct_identifier, &args.table, &args.primary_key);

            quote! {
                #[automatically_derived]
                impl #struct_identifier {
                    #full_update

                    #selects
                }

                impl crate::models::shared_traits::HasMBID for #struct_identifier {
                    fn get_mbid(&self) -> &str {
                        &self.mbid
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
