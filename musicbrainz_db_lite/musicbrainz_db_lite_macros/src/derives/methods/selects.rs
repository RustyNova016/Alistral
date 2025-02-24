use proc_macro2::TokenStream;
use syn::Ident;

pub fn impl_selects(_struct_name: &Ident, table_name: &str, _pk: &str) -> TokenStream {
    let sql = format!("SELECT * FROM `{}` WHERE id = ?", table_name);

    quote::quote! {
        /// Fetch a row by it's id.
        pub async fn find_by_id_column(conn: &mut sqlx::SqliteConnection, id: i64) -> Result<Option<Self>, sqlx::Error> {
            sqlx::query_as(#sql).bind(id).fetch_optional(conn).await
        }
    }
}
