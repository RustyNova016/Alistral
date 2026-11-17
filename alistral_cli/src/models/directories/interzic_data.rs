use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    pub fn interzic_data_and_create(&self) -> PathBuf {
        let dir = self.interzic_data();
        create_dir_all(&dir).unwrap();
        dir
    }

    pub fn interzic_data(&self) -> PathBuf {
        if let Some(path) = Self::get_env("INTERZIC_DATA_PATH") {
            return path;
        }

        if let Some(path) = systemd_directories::state_dir() {
            return path;
        }

        self.base_dirs.data_dir().join("interzic")
    }
}
