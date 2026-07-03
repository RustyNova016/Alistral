use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
pub struct ListenMetadata(pub HashMap<String, serde_json::Value>);

impl ListenMetadata {
    pub fn submission_client(&self) -> Option<&str> {
        self.0.get("submission_client").and_then(|val| val.as_str())
    }

    pub fn music_service_name(&self) -> Option<&str> {
        self.0
            .get("music_service_name")
            .and_then(|val| val.as_str())
    }
}
