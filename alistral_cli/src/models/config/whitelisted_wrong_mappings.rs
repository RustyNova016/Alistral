use serde::Deserialize;
use serde::Serialize;

use super::config_trait::ConfigFile;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WhilistedWrongMappings(Vec<WrongMapping>);

#[derive(Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
struct WrongMapping {
    msid: String,
    mbid: String,
}

impl WhilistedWrongMappings {
    pub fn add(&mut self, msid: String, mbid: String) {
        let new = WrongMapping { msid, mbid };

        if !self.0.contains(&new) {
            self.0.push(new);
        }
    }

    pub fn is_whitelisted(&self, msid: &String, mbid: &String) -> bool {
        let new = WrongMapping {
            msid: msid.to_string(),
            mbid: mbid.to_string(),
        };

        self.0.contains(&new)
    }
}

impl ConfigFile for WhilistedWrongMappings {
    fn file_name() -> &'static str {
        "wrong_mapping_whitelist.json"
    }
}
