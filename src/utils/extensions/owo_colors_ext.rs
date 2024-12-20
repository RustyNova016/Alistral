use core::fmt::Display;

use color_eyre::owo_colors::OwoColorize;

use crate::utils::cli::constants::CLEAR_UNTIL_END_OF_LINE;

pub trait AlistralColors: Display {
    fn true_color_tup(&self, color: (u8, u8, u8)) -> String {
        self.truecolor(color.0, color.1, color.2).to_string()
    }

    fn on_truecolor_tup(&self, color: (u8, u8, u8)) -> String {
        self.on_truecolor(color.0, color.1, color.2).to_string()
    }

    fn alistral_green(&self) -> String {
        self.truecolor(18, 198, 121).to_string()
    }

    fn on_alistral_green(&self) -> String {
        self.on_truecolor(18, 198, 121).to_string()
    }

    fn on_alistral_dark_green(&self) -> String {
        self.on_truecolor(0, 165, 93).to_string()
    }

    fn as_title(&self) -> String {
        format!(" {self} {CLEAR_UNTIL_END_OF_LINE}")
            .bold()
            .on_alistral_dark_green()
            .black()
            .to_string()
    }
}

impl<T: Display> AlistralColors for T {}
