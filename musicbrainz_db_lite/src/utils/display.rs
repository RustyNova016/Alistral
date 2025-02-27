#[cfg(feature = "pretty_format")]
use owo_colors::OwoColorize as _;

#[cfg(feature = "pretty_format")]
pub fn format_disambiguation(title: &str, disambiguation: &Option<String>) -> String {
    let dis = match disambiguation {
        None => "",
        Some(val) => {
            if !val.is_empty() {
                &format!(" ({})", &val).truecolor(175, 175, 175).to_string()
            } else {
                ""
            }
        }
    };

    format!("{title}{dis}")
}
