use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct RadioVariable {
    /// What variable to target
    pub targets: Vec<String>,

    /// Description of the variable
    pub description: Option<String>,

    /// Does it have a default?
    pub default: Option<Value>,

    /// Should it be hidden from info?
    #[serde(default)]
    pub hidden: bool,
}
