use core::fmt::Display;

use alistral_core::cli::colors::AlistralColors as _;

/// A cell in a top table
pub struct TopCell<T> {
    /// The data of the current slice
    pub current: Option<T>,
    /// The data of the previous slice
    pub previous: Option<T>,

    /// If true, show comparison between current and previous
    pub show_prev: bool,
}

impl<T> TopCell<T>
where
    T: Default + Clone + Display,
{
    pub fn new(current: Option<T>, previous: Option<T>, show_prev: bool) -> TopCell<T> {
        Self {
            current,
            previous,
            show_prev,
        }
    }

    pub fn format(&self, max_len_cur: usize, max_len_prev: usize) -> String {
        let cur = self.current.clone().unwrap_or_default();
        let prev = self.previous.clone().unwrap_or_default();

        if !self.show_prev {
            cur.to_string()
        } else {
            format!(
                "{cur:<width$} {}",
                format!("≪ {prev:>width$}", width = max_len_prev).true_color_tup((100, 100, 100)),
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
