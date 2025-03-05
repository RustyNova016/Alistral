use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use clap::Parser;
use futures::TryStreamExt;
use serde_json::Value;
use serde_json::json;
use tracing::debug;
use yumako_jams::client::YumakoClient;
use yumako_jams::json::radio::Radio;

use crate::api::clients::ALISTRAL_CLIENT;
use crate::api::clients::create_client;
use crate::tools::interzic::get_mapping::GetMappingCommand;

#[derive(Parser, Debug, Clone)]
pub struct RadioYumakoCommand {}

impl RadioYumakoCommand {
    pub async fn run(&self, conn: &mut sqlx::SqliteConnection) -> Result<(), crate::Error> {
        // Open the file in read-only mode with buffer.
        let file = File::open("./yumako_jams/exemples/simple_radio.json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let radio_schema: Radio = serde_json::from_reader(reader).unwrap();

        let mut vars = HashMap::new();
        vars.insert("username".to_string(), Value::String("RustyNova".to_string()));

        let client = YumakoClient {
            alistral_core: Arc::new(create_client().await),
        };

        debug!("Compiling radio");
        let mut radio = radio_schema.to_stream(&client, vars).unwrap();
        debug!("Compiled radio");

        for i in 0..5 {
            let track = radio.try_next().await.unwrap().unwrap();

            println!(
                "{}",
                track
                    .entity()
                    .pretty_format_with_credits(conn, &ALISTRAL_CLIENT.musicbrainz_db, true)
                    .await
                    .unwrap()
            )
        }

        Ok(())
    }
}
