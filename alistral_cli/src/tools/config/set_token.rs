use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json::Map;
use serde_json::Value;
use snafu::ResultExt;

use crate::interface::errors::friendly_error::FriendlyPanic;
use crate::interface::errors::friendly_error::FriendlyPanicSnafu;
use crate::interface::errors::friendly_error::GetFriendlyError;
use crate::models::config::AlistralConfig;
use crate::models::config::AlistralConfigError;
use crate::tools::bump::BumpCommandError;

/// Set the token of an user
#[derive(clap::Parser, Debug, Clone)]
pub struct ConfigSetTokenCommand {
    /// Name of the user to add the token
    username: String,

    /// User token
    token: String,
}

impl ConfigSetTokenCommand {
    pub async fn run(&self) -> Result<(), ConfigSetTokenCommandError> {
        let config_path = AlistralConfig::get_config_file_path().context(AlistralConfigSnafu)?;

        if config_path.extension().is_some_and(|ext| ext != "json")  {
            return UneditableConfigSnafu.fail();
        }

        let config = read_config(&config_path)?;
        match config {
            Value::Object(obj) => {
                obj.insert(, v)
            }
        }

        Ok(())
    }
}

fn read_config(config_path: &Path) -> Result<Value, ConfigSetTokenCommandError> {
    if !config_path.exists() {
        return Ok(Value::Object(Map::new()))
    }

    let mut file = File::open(&config_path).context(IOSnafu)?;
    let mut content = String::new();

    file.read_to_string(&mut content).context(IOSnafu)?;
    serde_json::from_str(&content).context(ConfigParsingSnafu)
}

#[derive(Debug, snafu::Snafu)]
pub enum ConfigSetTokenCommandError {
    AlistralConfigError {
        source: AlistralConfigError,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    IOError {
        source: std::io::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    ConfigParsingError {
        source: serde_json::Error,

        #[snafu(implicit)]
        location: snafu::Location,
    },

    UneditableConfigError {}
}

impl GetFriendlyError for ConfigSetTokenCommandError {
    fn get_friendly_error(&self) -> Option<FriendlyPanic> {
        match self {
            Self::UneditableConfigError { .. } => Some(
                FriendlyPanicSnafu {
                    title: "Error while writing the config".to_string(),
                    body: format!("Only json config files can be edited"),
                }
                .build(),
            ),
            Self::AlistralConfigError { source, .. } => source.get_friendly_error(),
        }
    }
}
