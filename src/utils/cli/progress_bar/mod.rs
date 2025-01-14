use std::ops::Deref;
use std::time::Duration;

use alistral_core::cli::logger::Logger;
use indicatif::{ProgressBar, ProgressStyle};

pub mod global_progress_bar;

pub struct ProgressBarCli {
    pg: ProgressBar,
}

impl ProgressBarCli {
    pub fn new<N: Into<u64>>(len: N, name: Option<&str>) -> Self {
        let mut progress_bar = ProgressBar::new(len.into());

        let mut style_string = "{wide_bar} {pos}/{len} | {eta_precise}".to_string();

        if let Some(name) = name {
            let name_style = format!("[{name}]");

            style_string = format!("{name_style} {style_string}");
        }

        progress_bar = progress_bar.with_style(
            ProgressStyle::with_template(&style_string).expect("Couldn't convert template schema"),
        );

        progress_bar.enable_steady_tick(Duration::from_secs(1));

        Logger::add_global_pg(progress_bar.clone());

        Self { pg: progress_bar }
    }
}

impl Deref for ProgressBarCli {
    type Target = ProgressBar;
    fn deref(&self) -> &Self::Target {
        &self.pg
    }
}

impl Drop for ProgressBarCli {
    fn drop(&mut self) {
        Logger::remove_global_pg(self.pg.clone());
    }
}
