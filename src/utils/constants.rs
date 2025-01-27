use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;

pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard config directory. Is your system an oddball one?")
        .config_local_dir()
        .to_path_buf();

    path.push("alistral");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create config directory");
    }

    path
});

pub static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = BaseDirs::new()
        .expect("Couldn't find the standard config directory. Is your system an oddball one?")
        .cache_dir()
        .to_path_buf();

    path.push("alistral");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create config directory");
    }

    path
});

pub static YT_SECRET_FILE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_credentials.json");

    path
});

pub static TOKENCACHE: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("youtube_tokens.json");

    path
});

pub static INTERZIC_DB: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.clone();

    path.push("interzic.db");

    path
});
