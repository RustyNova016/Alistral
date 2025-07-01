use crate::database::DEBUG_DB_LOCATION;
use crate::database::RELEASE_DB_LOCATION;
use std::fs;
use std::fs::remove_file;
use std::io;
use std::path::Path;

pub mod refresh_data;

pub fn delete_database(path: &Path) -> Result<(), crate::Error> {
    delete_or_not_found(path)?;
    delete_or_not_found(format!("{}-wal", path.to_string_lossy()))?;
    delete_or_not_found(format!("{}-shm", path.to_string_lossy()))?;

    Ok(())
}

fn delete_or_not_found<P: AsRef<Path>>(path: P) -> Result<(), crate::Error> {
    match remove_file(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(());
            }

            Err(crate::Error::DatabaseIo(err))
        }
    }
}

pub fn copy_to_debug() {
    delete_database(&DEBUG_DB_LOCATION).expect("Couldn't delete database");

    fs::copy(&*RELEASE_DB_LOCATION, &*DEBUG_DB_LOCATION).expect("Couldn't copy the database");
}
