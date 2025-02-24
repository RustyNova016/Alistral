use darling::{util::PathList, FromDeriveInput};

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(database), supports(struct_named))]
pub struct DatabaseAtribute {
    pub table: String,
    pub primary_key: String,
    pub ignore_insert_keys: PathList,
    pub ignore_update_keys: PathList,
}
