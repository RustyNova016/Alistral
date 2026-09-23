use core::ops::Deref as _;

use serde_json::Map;
use serde_json::Value;
use snafu::ResultExt as _;
use yumako_jams::RadioInputs;

use crate::models::config::config_trait::ConfigFile as _;
use crate::models::config::recording_timeout::RecordingTimeoutConfig;
use crate::tools::yumako_jam::run::error::RadioInputsParseSnafu;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;
use crate::utils::user_inputs::UserInputParser;

pub(super) fn get_radio_inputs(args: &str) -> Result<RadioInputs, YumakoRunCommandError> {
    let mut args = args.trim().to_string();

    if !args.starts_with('{') {
        args = format!("{{{args}");
    }

    if !args.ends_with('}') {
        args = format!("{args}}}");
    }

    let mut data: Map<String, Value> = json5::from_str(&args).context(RadioInputsParseSnafu)?;

    if !data.contains_key("username") {
        data.insert(
            "username".to_string(),
            Value::from(UserInputParser::username_or_default(&None)),
        );
    }

    // TODO: improve the config
    let config = RecordingTimeoutConfig::load().expect("Couldn't fetch the timeout config");
    let config = config.read_or_panic();

    if !data.contains_key("timeouts") {
        data.insert(
            "timeouts".to_string(),
            serde_json::to_value(config.deref().deref())
                .expect("Couldn't read the timeouts as a json Value"),
        );
    }

    Ok(RadioInputs::new(data))
}
