pub mod date_utils;
pub mod display;
pub mod macros;
pub mod querry_builder;
pub mod sqlx_utils;
#[cfg(test)]
pub mod tests;

pub(crate) fn strip_quotes(mut string: String) -> String {
    string.pop(); // remove last
    if !string.is_empty() {
        string.remove(0); // remove first
    }

    string
}
