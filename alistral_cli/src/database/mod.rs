use std::fs::{self};
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;

pub mod interfaces;
pub mod listenbrainz;

pub static RELEASE_DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard cache directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();

    path.push("alistral");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }
    path.push("data.db");

    path
});

pub static DEBUG_DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard cache directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();

    path.push("alistral");
    path.push("debug");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create cache directory");
    }
    path.push("debug_db.db");

    path
});

pub static DB_LOCATION: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(debug_assertions)]
    return DEBUG_DB_LOCATION.clone();

    #[cfg(not(debug_assertions))]
    return RELEASE_DB_LOCATION.clone();
});
