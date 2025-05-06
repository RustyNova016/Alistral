use core::fmt::Display;

use inquire::Select;
use strum::IntoEnumIterator;

pub fn select_enum<T: IntoEnumIterator + Display>(message: &str) -> Select<'_, T> {
    Select::new(message, T::iter().collect())
}
