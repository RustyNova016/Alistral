pub mod interzic_data;
use core::str::FromStr;
use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;

use directories::BaseDirs;

pub mod alistral_cache;
pub mod alistral_config;
pub mod alistral_data;
pub mod alistral_logs;
pub mod database_path;
pub mod yumako_jams_config;

pub const DIRECTORIES: LazyLock<Directories> = LazyLock::new(|| Directories::load());

pub struct Directories {
    //state_directory: PathBuf,
    base_dirs: BaseDirs,
}

impl Directories {
    pub fn load() -> Self {


        let base_dirs = BaseDirs::new()
            .expect("Couldn't find standard directories. Is your system an oddball one?");
        
        Self {
            base_dirs,
        }
    }

    pub fn get_env(name: &str) -> Option<PathBuf> {
        let var = env::var(name).ok()?;
        let path = PathBuf::from_str(&var).ok()?;

        if path.exists() {
            return Some(path);
        }

        None
    }
}
