use futures::StreamExt as _;
use snafu::ResultExt as _;
use tuillez::fatal_error::FatalError;
use yumako_jams::RadioStream;
use yumako_jams::models::radio_stream::radio_item::RadioItem;

use crate::ALISTRAL_CLIENT;
use crate::models::cli::radio::RadioExportTarget;
use crate::tools::yumako_jam::run::arguments::get_radio_inputs;
use crate::tools::yumako_jam::run::error::RadioCompilationSnafu;
use crate::tools::yumako_jam::run::error::YumakoRunCommandError;
use crate::tools::yumako_jam::run::export::export_radio;
use crate::tools::yumako_jam::run::get_radio::get_radio;

pub mod arguments;
pub mod error;
pub mod export;
pub mod get_radio;

#[derive(clap::Parser, Debug, Clone)]
pub struct YumakoRunCommand {
    /// The name of the radio
    radio_name: String,

    /// The arguments of the radio
    arguments: String,

    /// Where to send the radio
    #[clap(long)]
    target: RadioExportTarget,

    /// If applicable, the instance of the targeted service
    #[clap(long)]
    instance: Option<String>,

    /// The username of the user
    #[clap(long)]
    username: Option<String>,

    /// The token of the user
    #[clap(long)]
    token: Option<String>,
}

impl YumakoRunCommand {
    pub async fn run(&self) -> Result<(), YumakoRunCommandError> {
        let radio = get_radio(&self.radio_name)?;
        let inputs = get_radio_inputs(&self.arguments)?;

        let radio_name = radio.name.to_string();
        let radio_desc = radio.description.to_string();

        let radio = radio
            .to_stream(&ALISTRAL_CLIENT.yumako_jams, inputs)
            .context(RadioCompilationSnafu)?;

        let tracks = collect_radio(radio).await;

        let mut conn = ALISTRAL_CLIENT.get_conn().await;
        export_radio(
            &mut conn,
            &radio_name,
            &radio_desc,
            tracks,
            self.target.clone(),
            self.username.clone(),
            self.token.as_deref(),
            self.instance.as_deref(),
        )
        .await?;

        Ok(())
    }
}

async fn collect_radio(mut radio: RadioStream<'_>) -> Vec<RadioItem> {
    let mut clean_tracks = Vec::new();
    let mut error_count = 0;

    while let Some(track) = radio.next().await {
        match track {
            Ok(track) => clean_tracks.push(track),
            Err(err) => {
                tracing::error!("Encountered Error on playlist generation: {}", err);

                if error_count > 5 {
                    FatalError::new_string("Found too many errors. Aborting").panic();
                }

                error_count += 1;
            }
        }
    }

    clean_tracks
}
