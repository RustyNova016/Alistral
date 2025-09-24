use darling::{util::PathList, FromDeriveInput};

#[derive(FromDeriveInput, Clone)]
#[darling(attributes(database), supports(struct_named))]
pub struct DatabaseAtribute {
    pub table: String,
    pub primary_key: String,
    #[allow(dead_code)]
    pub ignore_insert_keys: PathList,
    #[allow(dead_code)]
    pub ignore_update_keys: PathList,
}
