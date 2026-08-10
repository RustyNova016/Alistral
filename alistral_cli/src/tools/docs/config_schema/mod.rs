use schemars::schema_for;

use crate::models::config::Config;

/// Create the json schema of the config
#[derive(clap::Parser, Debug, Clone)]
pub struct DocsConfigSchemaCommand {}

impl DocsConfigSchemaCommand {
    pub fn run(&self) {
        let schema = schema_for!(Config);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
