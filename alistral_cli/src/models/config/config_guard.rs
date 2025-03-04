use core::ops::Deref;
use core::ops::DerefMut;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;
use std::thread::panicking;

use super::config_trait::ConfigFile;

pub struct ConfigGuard<T: ConfigFile>(RwLock<T>);

impl<T: ConfigFile> ConfigGuard<T> {
    pub fn new(config: T) -> Self {
        Self(RwLock::new(config))
    }

    pub fn read(
        &self,
    ) -> Result<RwLockReadGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockReadGuard<'_, T>>>
    {
        self.0.read()
    }

    pub fn read_or_panic(&self) -> RwLockReadGuard<'_, T> {
        self.read().expect("Lock poisoned")
    }

    pub fn write(
        &self,
    ) -> Result<ConfigWriteGuard<'_, T>, std::sync::PoisonError<std::sync::RwLockWriteGuard<'_, T>>>
    {
        self.0.write().map(|guard| ConfigWriteGuard(guard))
    }

    pub fn write_or_panic(&self) -> ConfigWriteGuard<'_, T> {
        self.write().expect("Lock poisoned")
    }
}

pub struct ConfigWriteGuard<'l, T: ConfigFile>(RwLockWriteGuard<'l, T>);

impl<'l, T: ConfigFile> Deref for ConfigWriteGuard<'l, T> {
    type Target = RwLockWriteGuard<'l, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ConfigFile> DerefMut for ConfigWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ConfigFile> Drop for ConfigWriteGuard<'_, T> {
    fn drop(&mut self) {
        if panicking() {
            return;
        }

        self.0.save().unwrap();
    }
}
