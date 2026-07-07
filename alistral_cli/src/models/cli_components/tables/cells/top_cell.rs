use core::fmt::Display;

use alistral_core::cli::colors::AlistralColors as _;

/// A cell in a top table
#[derive(Debug, Default, Clone)]
pub struct TopCell<T> {
    /// The data of the current slice
    pub current: Option<T>,
    /// The data of the previous slice
    pub previous: Option<T>,
}

impl<T> TopCell<T>
where
    T: Default + Clone + Display,
{
    pub fn new(current: Option<T>, previous: Option<T>) -> Self {
        Self { current, previous }
    }

    pub fn format(&self, max_len_cur: usize, max_len_prev: usize, show_prev: bool) -> String {
        let cur = self.current.clone().unwrap_or_default();
        let prev = self.previous.clone().unwrap_or_default();

        if !show_prev {
            cur.to_string()
        } else {
            format!(
                "{cur:<width$} {}",
                format!("≪ {prev:>max_len_prev$}").true_color_tup((100, 100, 100)),
                width = max_len_cur,
            )
        }
    }

    // pub fn max_lengths(cells: &[Self]) -> (usize, usize) {
    //     (
    //         cells
    //             .iter()
    //             .map(|e| e.current.clone().unwrap_or_default().to_string().len())
    //             .max()
    //             .unwrap_or_default(),
    //         cells
    //             .iter()
    //             .map(|e| e.previous.clone().unwrap_or_default().to_string().len())
    //             .max()
    //             .unwrap_or_default(),
    //     )
    // }
}
