use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    #[cfg(not(test))]
    pub fn alistral_cache_and_create(&self) -> PathBuf {
        use std::fs::create_dir_all;

        let dir = self.alistral_cache();
        create_dir_all(&dir).unwrap();
        dir
    }

    #[cfg(not(test))]
    pub fn alistral_cache(&self) -> PathBuf {
        #[cfg(debug_assertions)]
        return self.alistral_debug_cache();

        #[cfg(not(debug_assertions))]
        return self.alistral_release_cache();
    }

    pub fn alistral_release_cache(&self) -> PathBuf {
        if let Some(path) = Self::get_env("ALISTRAL_CACHE_PATH") {
            return path;
        }

        if let Some(path) = systemd_directories::config_dir() {
            return path;
        }

        self.base_dirs.cache_dir().join("alistral")
    }

    pub fn alistral_debug_cache(&self) -> PathBuf {
        self.alistral_release_cache().join("debug")
    }
}
