use core::ops::Deref;
use core::ops::DerefMut;
use std::sync::RwLock;
// use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::thread::panicking;

use super::DataFile;

pub struct FileGuard<T: DataFile>(RwLock<T>);

impl<T: DataFile> FileGuard<T> {
    pub fn new(config: T) -> Self {
        Self(RwLock::new(config))
    }

    // pub fn read(
    //     &self,
    // ) -> Result<RwLockReadGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>>
    // {
    //     self.0.read()
    // }

    // pub fn read_or_panic(&self) -> RwLockReadGuard<'_, T> {
    //     self.read().expect("Lock poisoned")
    // }

    pub fn write(
        &self,
    ) -> Result<FileWriteGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>>
    {
        self.0.write().map(|guard| FileWriteGuard(guard))
    }

    // pub fn write_or_panic(&self) -> FileWriteGuard<'_, T> {
    //     self.write().expect("Lock poisoned")
    // }
}

pub struct FileWriteGuard<'l, T: DataFile>(RwLockWriteGuard<'l, T>);

impl<'l, T: DataFile> Deref for FileWriteGuard<'l, T> {
    type Target = RwLockWriteGuard<'l, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: DataFile> DerefMut for FileWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: DataFile> Drop for FileWriteGuard<'_, T> {
    fn drop(&mut self) {
        if panicking() {
            return;
        }

        self.0.save().unwrap();
    }
}
