
use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    // pub fn alistral_logs_and_create(&self) -> PathBuf {
    //     let dir = self.alistral_logs();
    //     create_dir_all(&dir).unwrap();
    //     dir
    // }

    pub fn alistral_logs(&self) -> PathBuf {
        if let Some(path) = Self::get_env("ALISTRAL_LOGS_PATH") {
            return path;
        }

        if let Some(path) = systemd_directories::logs_dir() {
            return path;
        }

        self.base_dirs.data_dir().join("alistral/logs")
    }
}
