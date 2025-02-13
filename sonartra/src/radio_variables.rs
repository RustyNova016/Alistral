use std::collections::HashMap;

use serde_json::Value;

/// Represent all the variable of a radio
pub struct RadioVariables {
    values: HashMap<String, Value>,
}

impl RadioVariables {
    pub fn new_with_aliases(
        data: HashMap<String, Value>,
        aliases: HashMap<String, String>,
    ) -> Self {
        let mut inner = HashMap::new();

        for (alias, target) in aliases.clone() {
            match data.get(&alias) {
                None => {}
                Some(val) => {
                    inner.insert(target, val.clone());
                }
            }
        }

        for (target, value) in data {
            // Ignore the aliased values
            if aliases.contains_key(&target) {
                continue;
            }

            inner.insert(target, value);
        }

        Self { values: inner }
    }

    /// This returns the variables of a layer
    pub fn get_layer_variables(&self, layer_name: &str) -> HashMap<String, Value> {
        let mut out = HashMap::new();
        for (key, value) in &self.values {
            let (domain, var_name) = key.split_once(".").expect("msg"); //TODO

            if domain == layer_name {
                out.insert(var_name.to_string(), value.clone());
            }
        }

        out
    }
}
