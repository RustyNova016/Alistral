use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct RadioInput {
    pub targets: Vec<String>,
    title: Option<String>,
    description: Option<String>,
    #[serde(default)]
    required: bool,
    pub default: Option<Value>,
    #[serde(default)]
    hidden: bool,
}
