use tuillez::OwoColorize as _;

pub struct LookupCompArrow;

impl LookupCompArrow {
    /// Get a comparison arrow. ▲ (green) if a > b, ▼ (red) if a < b, - if equal
    pub fn comp_asc<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▲".green().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▼".red().to_string()
        }
    }

    /// Get a comparison arrow. ▲ (red) if a > b, ▼ (green) if a < b, - if equal
    pub fn comp_desc<T: PartialOrd>(a: T, b: T) -> String {
        if a > b {
            "▲".red().to_string()
        } else if a == b {
            "-".bright_black().to_string()
        } else {
            "▼".green().to_string()
        }
    }
}
