use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    #[cfg(feature = "radio")]
    pub fn alistral_data_and_create(&self) -> PathBuf {
        let dir = self.alistral_data();
        create_dir_all(&dir).unwrap();
        dir
    }

    #[cfg(feature = "radio")]
    pub fn alistral_data(&self) -> PathBuf {
        if let Some(path) = Self::get_env("ALISTRAL_DATA_PATH") {
            return path;
        }

        if let Some(path) = systemd_directories::state_dir() {
            return path;
        }

        self.base_dirs.data_dir().join("alistral")
    }
}
