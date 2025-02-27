use core::fmt::Display;

/// Rename an hyperlink with a text
pub fn hyperlink_rename(text: &impl Display, link: &str) -> String {
    let osc8: &str = "\x1b]8";
    let st: &str = "\x1b\\";
    format!(r"{osc8};;{link}{st}{text}{osc8};;{st}")
}
