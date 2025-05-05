pub mod date_utils;
pub mod display;
pub mod macros;
pub mod querry_builder;
pub mod sqlx_utils;
pub mod testing_framework;
#[cfg(test)]
pub mod tests;

pub(crate) fn strip_quotes(mut string: String) -> String {
    string.pop(); // remove last
    if !string.is_empty() {
        string.remove(0); // remove first
    }

    string
}

pub async fn force_write_transaction(conn: &mut sqlx::SqliteConnection) {
    sqlx::query("UPDATE `_sqlx_migrations` SET version = 99 WHERE version = 0 ")
        .execute(conn)
        .await
        .unwrap();
}
