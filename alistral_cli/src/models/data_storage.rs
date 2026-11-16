use serde::Deserialize;
use serde::Serialize;

use crate::models::directories::DIRECTORIES;
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
        let mut path = DIRECTORIES.alistral_data_and_create();
        path.push("data_storage.txt");
        path
    }
}
