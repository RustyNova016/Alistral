use std::fs;
use std::fs::File;
use std::io;
use std::path::PathBuf;

use directories::BaseDirs;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::config_guard::ConfigGuard;

pub trait ConfigFile: Serialize + DeserializeOwned + Default {
    fn file_name() -> &'static str;

    fn path_to_config() -> PathBuf {
        let mut path = BaseDirs::new()
            .expect("Couldn't find standard directory. Is your system an oddball one?")
            .config_dir()
            .to_path_buf();
        path.push("alistral");
        fs::create_dir_all(&path).expect("Couldn't create config directory");
        path.push(Self::file_name());
        path
    }

    fn path_to_config_legacy() -> PathBuf {
        let mut path = BaseDirs::new()
            .expect("Couldn't find standard directory. Is your system an oddball one?")
            .config_dir()
            .to_path_buf();
        path.push("listenbrainz_cli_tools");
        fs::create_dir_all(&path).expect("Couldn't create config directory");
        path.push(Self::file_name());
        path
    }

    fn save(&self) -> Result<(), crate::Error> {
        let config_file = File::create(Self::path_to_config().as_path())
            .map_err(crate::Error::ConfigFileCreation)?;
        serde_json::to_writer_pretty(config_file, self).map_err(crate::Error::ConfigFileWrite)?;
        Ok(())
    }

    fn load_unguarded() -> Result<Self, crate::Error> {
        match Self::get_config_reader() {
            Ok(Some(data)) => {
                serde_json::from_reader(data).map_err(crate::Error::ConfigDeserialization)
            }
            Ok(None) => Ok(Self::default()),
            Err(err) => Err(crate::Error::ConfigRead(err)),
        }
    }

    fn get_config_reader() -> io::Result<Option<File>> {
        match File::open(Self::path_to_config().as_path()) {
            Ok(reader) => Ok(Some(reader)),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    match File::open(Self::path_to_config_legacy().as_path()) {
                        Ok(reader) => Ok(Some(reader)),
                        Err(err) => match err.kind() {
                            io::ErrorKind::NotFound => Ok(None),
                            _ => Err(err),
                        },
                    }
                }
                _ => Err(err),
            },
        }
    }

    fn load() -> Result<ConfigGuard<Self>, crate::Error> {
        Ok(ConfigGuard::new(Self::load_unguarded()?))
    }
}
