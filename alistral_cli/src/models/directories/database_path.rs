use std::path::PathBuf;

use crate::models::directories::Directories;

impl Directories {
    pub fn musicbrainz_cache_path(&self) -> PathBuf {
        #[cfg(test)]
        return get_test_db();

        #[cfg(not(test))]
        self.alistral_cache_and_create()
            .join("musicbrainz_cache.db")
    }

    pub fn musicbrainz_release_cache_path(&self) -> PathBuf {
        self.alistral_release_cache().join("musicbrainz_cache.db")
    }

    pub fn musicbrainz_debug_cache_path(&self) -> PathBuf {
        self.alistral_debug_cache().join("musicbrainz_cache.db")
    }

    pub fn interzic_database_path(&self) -> PathBuf {
        self.interzic_data_and_create().join("interzic_data.db")
    }
}

#[cfg(test)]
fn get_test_db() -> PathBuf {
    use core::str::FromStr as _;
    use std::fs;

    use uuid::Uuid;

    let mut path = PathBuf::from_str("./target/db/").unwrap();

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }

    path.push(format!("{}.db", Uuid::now_v7()));

    path
}
