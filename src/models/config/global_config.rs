use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::LazyLock;
use std::thread::panicking;

use tokio::sync::RwLock;
use tokio::sync::RwLockReadGuard;
use tokio::sync::RwLockWriteGuard;

use super::config_trait::ConfigFile as _;
use super::Config;

pub(crate) static CONFIG: LazyLock<GlobalConfig> = LazyLock::new(GlobalConfig::load);

pub struct GlobalConfig {
    config: Arc<RwLock<Config>>,
}

impl GlobalConfig {
    pub fn load() -> Self {
        Self {
            config: Arc::new(RwLock::new(
                Config::load_unguarded().expect("Couldn't load the configuration file"),
            )),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<Config> {
        self.config.read().await
    }

    pub async fn write(&self) -> GlobalConfigGuard<'_> {
        let guard = self.config.write().await;

        GlobalConfigGuard { config: guard }
    }
}

pub struct GlobalConfigGuard<'a> {
    pub(self) config: RwLockWriteGuard<'a, Config>,
}

impl Drop for GlobalConfigGuard<'_> {
    fn drop(&mut self) {
        if panicking() {
            return;
        }

        self.config.save().expect("Couldn't save config");
    }
}

impl Deref for GlobalConfigGuard<'_> {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.config.deref()
    }
}

impl DerefMut for GlobalConfigGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.config.deref_mut()
    }
}
