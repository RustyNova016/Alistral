use core::fmt::Display;

use inquire::Select;

pub struct SelectItem<T> {
    text: String,
    data: T,
}

impl<T> SelectItem<T> {
    pub fn new(text: String, data: T) -> Self {
        Self { data, text }
    }

    pub fn into_data(self) -> T {
        self.data
    }
}

impl<T> Display for SelectItem<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)?;

        Ok(())
    }
}

pub fn select_associated<T, F: Fn(&T) -> String>(
    message: &str,
    choices: Vec<T>,
    f: F,
) -> Select<'_, SelectItem<T>> {
    let items = choices
        .into_iter()
        .map(|t| SelectItem::new(f(&t), t))
        .collect();

    Select::new(message, items)
}
