use std::fs::File;
use std::io;
use std::path::PathBuf;

use file_guard::FileGuard;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub mod file_guard;

pub trait DataFile: Serialize + DeserializeOwned + Default {
    fn path() -> PathBuf;

    fn load_unguarded() -> Result<Self, crate::Error> {
        match File::open(Self::path().as_path()) {
            Ok(data) => {
                serde_json::from_reader(data).map_err(crate::Error::ConfigLoadDeserializationError)
            }
            Err(err) => {
                if err.kind() == io::ErrorKind::NotFound {
                    Ok(Self::default())
                } else {
                    Err(crate::Error::ConfigLoadError(err))
                }
            }
        }
    }

    fn load() -> Result<FileGuard<Self>, crate::Error> {
        Ok(FileGuard::new(Self::load_unguarded()?))
    }

    fn save(&self) -> Result<(), crate::Error> {
        let file: File =
            File::create(Self::path().as_path()).map_err(crate::Error::ConfigFileCreationError)?;
        serde_json::to_writer_pretty(file, self).map_err(crate::Error::ConfigFileWriteError)?;
        Ok(())
    }
}
