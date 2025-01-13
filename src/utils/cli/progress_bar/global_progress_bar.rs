use core::ops::Deref;
use core::sync::atomic::{AtomicU64, Ordering};
use core::time::Duration;
use std::sync::{Arc, RwLock};

use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::Lazy;

use crate::utils::logger::Logger;

pub static PG_FETCHING: Lazy<GlobalProgressBar<'static>> =
    Lazy::new(|| GlobalProgressBar::new("Fetching MBIDs".to_string()));

pub struct GlobalProgressBar<'a> {
    pg: ProgressBar,
    submitters: RwLock<Vec<Arc<SubmitterTask<'a>>>>,
    id_counter: AtomicU64,
}

impl<'a> GlobalProgressBar<'a> {
    pub fn new(text: String) -> Self {
        let mut progress_bar = ProgressBar::new(0);

        let mut style_string = "{wide_bar} {pos}/{len} | {eta_precise}".to_string();
        style_string = format!("[{text}] {style_string}");

        progress_bar = progress_bar.with_style(
            ProgressStyle::with_template(&style_string).expect("Couldn't convert template schema"),
        );

        progress_bar.enable_steady_tick(Duration::from_secs(1));

        Self {
            pg: progress_bar,
            submitters: RwLock::new(Vec::new()),
            id_counter: AtomicU64::new(0),
        }
    }

    /// Show the progressbar on the cli
    fn show(&self) {
        Logger::add_global_pg(self.pg.clone());
    }

    fn hide(&self) {
        self.pg.tick();
        Logger::remove_global_pg(self.pg.clone());
        self.pg.set_position(0);
    }

    pub(self) fn add_submiter(&self, submiter: SubmitterTask<'a>) -> Arc<SubmitterTask<'a>> {
        let submitter_id = submiter.id;
        let mut list = self.submitters.write().unwrap();
        list.push(Arc::new(submiter));

        if list.len() == 1 {
            self.show();
        }

        self.pg.set_length(list.iter().map(|s| s.count).sum());

        list.iter()
            .find(|s| s.id == submitter_id)
            .expect("The value should be inserted above")
            .clone()
    }

    pub(self) fn remove_submiter(&self, submiter: &Arc<SubmitterTask<'a>>) {
        let mut list = self.submitters.write().unwrap();
        list.retain(|saved_submiter| saved_submiter.id != submiter.id);

        if list.len() == 0 {
            self.hide();
        }
    }

    pub fn inc(&self, delta: u64) {
        self.pg.inc(delta);
    }

    #[must_use]
    pub fn get_submitter(&'a self, count: u64) -> SubmitterGuard<'a> {
        let id = self.id_counter.fetch_add(1, Ordering::Relaxed);
        let task = SubmitterTask::new(id, self, count);
        let arced = self.add_submiter(task);

        SubmitterGuard(arced)
    }
}

pub struct SubmitterTask<'a> {
    pub(self) id: u64,
    pub(self) bar: &'a GlobalProgressBar<'a>,
    pub(self) count: u64,
}

impl<'a> SubmitterTask<'a> {
    pub(self) fn new(id: u64, bar: &'a GlobalProgressBar<'a>, count: u64) -> Self {
        Self { id, bar, count }
    }

    pub fn inc(&self, delta: u64) {
        self.bar.inc(delta);
    }

    pub fn set_count(&self, count: u64) {
        self.bar.pg.set_length(count);
    }
}

pub struct SubmitterGuard<'pg>(pub(self) Arc<SubmitterTask<'pg>>);

impl<'pg> Deref for SubmitterGuard<'pg> {
    type Target = Arc<SubmitterTask<'pg>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for SubmitterGuard<'_> {
    fn drop(&mut self) {
        self.0.bar.remove_submiter(&self.0);
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::thread;

    use super::*;

    #[test]
    fn it_works() {
        let ten_millis = time::Duration::from_millis(1000);
        let pg = GlobalProgressBar::new("Test".to_string());

        let subm = pg.get_submitter(5);

        for _ in 0..5 {
            thread::sleep(ten_millis);
            subm.inc(1);
        }

        thread::sleep(ten_millis);
        let subm2 = pg.get_submitter(5);
        thread::sleep(ten_millis);
        drop(subm);

        for _ in 0..5 {
            thread::sleep(ten_millis);
            subm2.inc(1);
        }

        thread::sleep(ten_millis);
        drop(subm2);

        thread::sleep(ten_millis);
        let subm = pg.get_submitter(5);
        thread::sleep(ten_millis * 2);
        drop(subm);
        thread::sleep(ten_millis);
    }
}
