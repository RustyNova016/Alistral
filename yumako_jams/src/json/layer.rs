use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::aliases::LayerResult;
use crate::aliases::RadioStream;
use crate::client::YumakoClient;
use crate::modules::filters::booleans::AndFilter;
use crate::modules::filters::cooldown::CooldownFilter;
use crate::modules::filters::minimum_listens::MinimumListenFilter;
use crate::modules::filters::timeout::TimeoutFilter;
use crate::modules::radio_module::RadioModule;
use crate::modules::scores::listenrate::ListenRateScorer;
use crate::modules::scores::overdue::OverdueDurationScorer;
use crate::modules::scores::sort::SortModule;
use crate::modules::seeders::listen_seeder::ListenSeeder;
use crate::radio_variables::RadioVariables;

/// A layer represent a step in the radio processing. It calls a module based on the step type
#[derive(Serialize, Deserialize, Clone)]
pub struct Layer {
    id: String,
    step_type: String,

    /// The default variables for the layer
    #[serde(default)]
    inputs: HashMap<String, Value>,
}

impl Layer {
    pub fn create_step<'a>(
        self,
        client: &'a YumakoClient,
        stream: RadioStream<'a>,
        radio_variables: &RadioVariables,
    ) -> LayerResult<'a> {
        let variables = radio_variables.get_layer_variables(&self.id);

        match self.step_type.as_str() {
            "and_filter" => {
                AndFilter::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "cooldown_filter" => {
                CooldownFilter::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "listen_seeder" => {
                ListenSeeder::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "listenrate_scorer" => {
                ListenRateScorer::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "minimum_listen_filter" => {
                MinimumListenFilter::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "sort_module" => {
                SortModule::create(self.inputs, variables)?.create_stream(stream, client)
            }
            "timeout_filter" => {
                TimeoutFilter::create(self.inputs, variables)?.create_stream(stream, client)
            },
            "overdue_duration_scorer" => {
                OverdueDurationScorer::create(self.inputs, variables)?.create_stream(stream, client)
            }
            _ => {
                panic!("Wrong type") // TODO: Proper error
            }
        }
    }
}
