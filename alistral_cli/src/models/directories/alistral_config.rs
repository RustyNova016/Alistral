use std::fs::create_dir_all;
use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    pub fn alistral_config_and_create(&self) -> PathBuf {
        let dir = self.alistral_config();
        create_dir_all(&dir).unwrap();
        dir
    }

    pub fn alistral_config(&self) -> PathBuf {
        if let Some(path) = Self::get_env("ALISTRAL_CONFIG_PATH") {
            return path;
        }

        if let Some(path) = systemd_directories::config_dir() {
            return path;
        }

        self.base_dirs.config_dir().join("alistral")
    }
}
