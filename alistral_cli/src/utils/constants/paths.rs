use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;

fn get_basedirs() -> BaseDirs {
    BaseDirs::new()
        .expect("Couldn't find the standard config directory. Is your system an oddball one?")
}

fn create_alistral_dir(path: &Path) -> PathBuf {
    let mut path = path.to_path_buf();
    path.push("alistral");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create config directory");
    }

    path
}

pub static CONFIG_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| create_alistral_dir(get_basedirs().config_dir()));

pub static CACHE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| create_alistral_dir(get_basedirs().cache_dir()));

pub static LOG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let mut path = CONFIG_DIR.to_path_buf();
    path.push("logs");

    if !fs::exists(&path).unwrap() {
        fs::create_dir_all(&path).expect("Couldn't create config directory");
    }

    path
});
