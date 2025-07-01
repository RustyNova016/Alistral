use std::fs;

use directories::BaseDirs;
use serde::Deserialize;
use serde::Serialize;

use crate::utils::data_file::DataFile;

/// Storage for data other than configurations
#[derive(Serialize, Deserialize)]
pub struct DataStorage {
    playlist_count: u64,
}

impl DataStorage {
    #[cfg(feature = "radio")]
    /// Increment the playlist count, returning the old count for usage
    pub fn incr_playlist_count(&mut self) -> u64 {
        let count = self.playlist_count;
        self.playlist_count += 1;
        count
    }
}

impl Default for DataStorage {
    fn default() -> Self {
        Self { playlist_count: 1 }
    }
}

impl DataFile for DataStorage {
    fn path() -> std::path::PathBuf {
        let mut path = BaseDirs::new()
            .expect("Couldn't find standard directory. Is your system an oddball one?")
            .data_local_dir()
            .to_path_buf();
        path.push("alistral");
        fs::create_dir_all(&path).expect("Couldn't create config directory");
        path.push("data_storage.txt");
        path
    }
}
