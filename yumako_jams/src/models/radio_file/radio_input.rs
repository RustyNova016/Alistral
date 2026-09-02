use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct RadioInput {
    /// What variable to target
    pub targets: Vec<String>,

    /// The name of the variable
    title: Option<String>,

    /// Description of the variable
    description: Option<String>,

    /// Is it required?
    #[serde(default)]
    required: bool,

    /// Does it have a default?
    pub default: Option<Value>,

    /// Should it be hidden from help?
    #[serde(default)]
    hidden: bool,
}
