use core::fmt::Display;
use std::sync::Arc;
use std::sync::Mutex;

use indicatif::MultiProgress;
use indicatif::ProgressBar;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize as _;

use super::colors::AlistralColors as _;

pub struct Logger {
    print_override: Option<MultiProgress>, //TODO: Keep bar all the time?
    bar_count: u32,
}

pub static STATIC_LOGGER: Lazy<Arc<Mutex<Logger>>> =
    Lazy::new(|| Arc::new(Mutex::new(Logger::new())));

impl Logger {
    pub fn new() -> Self {
        Self {
            print_override: None,
            bar_count: 0,
        }
    }

    pub fn tick(&self) {
        if let Some(mpg) = &self.print_override {
            mpg.suspend(|| 0);
        }
    }

    pub fn add_bar(&mut self, pg: ProgressBar) {
        self.bar_count += 1;

        match &self.print_override {
            Some(mpg) => {
                mpg.add(pg);
            }
            None => {
                let mpg = MultiProgress::new();
                mpg.add(pg);
                self.print_override = Some(mpg);
            }
        }
    }

    pub fn remove_bar(&mut self, pg: ProgressBar) {
        self.bar_count -= 1;
        if let Some(mpg) = &self.print_override {
            mpg.remove(&pg);
            mpg.clear().expect("TODO: panic message");
        }

        if self.bar_count == 0 {
            self.print_override = None;
        }
    }

    fn print<T: Display + AsRef<str>>(&self, string: T) {
        if let Some(overide) = &self.print_override {
            overide.println(string).expect("Couldn't print");
        } else {
            println!("{string}");
        }
    }

    pub fn println_cli<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[Alistral]".alistral_green(), string));
    }

    pub fn println_lis<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[Listenbrainz]".blue(), string));
    }

    pub fn println_mus<T: Display>(&self, string: T) {
        self.print(format!("{} {}", "[MusicBrainz]".bright_magenta(), string));
    }

    pub fn add_global_pg(pg: ProgressBar) {
        let static_clone = STATIC_LOGGER.clone();
        let mut logger = static_clone.lock().unwrap();
        logger.add_bar(pg);
    }

    pub fn remove_global_pg(pg: ProgressBar) {
        let static_clone = STATIC_LOGGER.clone();
        let mut logger = static_clone.lock().unwrap();
        logger.remove_bar(pg);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

pub fn println_cli<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_cli(string);
}

pub fn println_cli_warn<T: Display>(string: T) {
    println_cli(format!("[Warning] {string}").yellow());
}

pub fn println_cli_info<T: Display>(string: T) {
    println_cli(format!("[Info] {string}").yellow());
}

pub fn println_lis<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_lis(string);
}

pub fn println_mus<T: Display>(string: T) {
    let static_clone = STATIC_LOGGER.clone();
    let logger = static_clone.lock().unwrap();
    logger.println_mus(string);
}
