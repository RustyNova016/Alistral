use core::cell::LazyCell;
use std::sync::LazyLock;

use color_eyre::owo_colors::OwoColorize;

pub mod recordings;

pub(self) static ARROW_UP_GREEN: LazyLock<String> = LazyLock::new(|| "▲".green().to_string());
pub(self) static ARROW_UP_RED: LazyLock<String> = LazyLock::new(|| "▲".red().to_string());
pub(self) static ARROW_DOWN_GREEN: LazyLock<String> = LazyLock::new(|| "▼".green().to_string());
pub(self) static ARROW_DOWN_RED: LazyLock<String> = LazyLock::new(|| "▼".red().to_string());
pub(self) static DASH_GREY: LazyLock<String> = LazyLock::new(|| "-".bright_black().to_string());
