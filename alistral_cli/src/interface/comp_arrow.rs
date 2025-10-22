use core::cmp::Ordering;

use tuillez::OwoColorize as _;

pub struct ComparisonArrow;

impl ComparisonArrow {
    /// Get a comparison arrow. ▲ (green) if a > b, ▼ (red) if a < b, - if equal
    pub fn greater_is_better<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▲".green().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▼".red().to_string()
        }
    }

    /// Get a comparison arrow. ▲ (red) if a > b, ▼ (green) if a < b, - if equal
    pub fn lesser_is_better<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▲".red().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▼".green().to_string()
        }
    }

        /// Get a comparison arrow. ▼ (green) if a > b, ▲ (red) if a < b, - if equal
    pub fn greater_is_better_inv<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▼".green().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▲".red().to_string()
        }
    }

    /// Get a comparison arrow. ▼ (red) if a > b, ▲ (green) if a < b, - if equal
    pub fn lesser_is_better_inv<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▼".red().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▲".green().to_string()
        }
    }
}
