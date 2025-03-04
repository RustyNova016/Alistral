use crate::models::error::Error;
use derive_getters::Getters;
use serde::Deserialize;
use serde::Serialize;

use super::Config;

#[derive(Debug, Serialize, Deserialize, Getters, Default)]
pub struct MapperConfig {
    /// List of recordings that shouldn't be proposed to be mapped
    pub(super) backlisted: Vec<String>,
}

impl Config {
    pub fn add_blacklisted_msid(&mut self, msid: String) -> Result<(), Error> {
        let mapper_config = self.mapper.get_or_insert_default();
        mapper_config.backlisted.push(msid);

        Ok(())
    }

    pub fn remove_blacklisted_msid(&mut self, msid: &String) -> Result<(), Error> {
        let mapper_config = self.mapper.get_or_insert_default();
        mapper_config.backlisted.retain(|id| id != msid);

        Ok(())
    }
}
