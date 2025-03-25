use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

use crate::RadioStream;
use crate::client::YumakoClient;
use crate::modules::filters::booleans::AndFilter;
use crate::modules::filters::cooldown::CooldownFilter;
use crate::modules::filters::listens::ListenFilter;
use crate::modules::filters::timeout::TimeoutFilter;
use crate::modules::listen_data::clear_listens::ClearListens;
use crate::modules::listen_data::last_listens::LatestListens;
use crate::modules::listen_data::listen_interval::ListenInterval;
use crate::modules::mappers::artist_discography::ArtistDiscographyMapper;
use crate::modules::radio_module::LayerResult;
use crate::modules::radio_module::RadioModule;
use crate::modules::scores::bump::BumpScore;
use crate::modules::scores::listenrate::ListenRateScorer;
use crate::modules::scores::overdue_count::OverdueCountScorer;
use crate::modules::scores::overdue_duration::OverdueDurationScorer;
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
        let variables = radio_variables.get_layer_variables(&self.id)?;

        match self.step_type.as_str() {
            "and_filter" => AndFilter::create(&self, variables)?.create_stream(stream, client),
            "artist_discography_mapper" => {
                ArtistDiscographyMapper::create(&self, variables)?.create_stream(stream, client)
            }
            "bumps_score" => BumpScore::create(&self, variables)?.create_stream(stream, client),
            "clear_listens" => {
                ClearListens::create(&self, variables)?.create_stream(stream, client)
            }
            "cooldown_filter" => {
                CooldownFilter::create(&self, variables)?.create_stream(stream, client)
            }
            "latest_listens" => {
                LatestListens::create(&self, variables)?.create_stream(stream, client)
            }
            "listen_filter" => {
                ListenFilter::create(&self, variables)?.create_stream(stream, client)
            }
            "listen_interval" => {
                ListenInterval::create(&self, variables)?.create_stream(stream, client)
            }
            "listen_seeder" => {
                ListenSeeder::create(&self, variables)?.create_stream(stream, client)
            }
            "listenrate_scorer" => {
                ListenRateScorer::create(&self, variables)?.create_stream(stream, client)
            }
            "sort_module" => SortModule::create(&self, variables)?.create_stream(stream, client),
            "timeout_filter" => {
                TimeoutFilter::create(&self, variables)?.create_stream(stream, client)
            }
            "overdue_count_scorer" => {
                OverdueCountScorer::create(&self, variables)?.create_stream(stream, client)
            }
            "overdue_duration_scorer" => {
                OverdueDurationScorer::create(&self, variables)?.create_stream(stream, client)
            }
            _ => Err(crate::Error::UnknownStepTypeError(
                self.step_type.to_string(),
            )),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn inputs(&self) -> &HashMap<String, Value> {
        &self.inputs
    }
}
