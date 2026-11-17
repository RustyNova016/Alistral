// Apparently I nest my functions too much :P
#![recursion_limit = "256"]

use std::process::ExitCode;

use clap::Parser as _;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::interface::errors::friendly_error::GetFriendlyError as _;
use crate::interface::tracing::init_tracer;
use models::cli::Cli;

pub(crate) mod api;
pub(crate) mod database;
pub(crate) mod datastructures;
pub(crate) mod interface;
pub(crate) mod models;
#[cfg(test)]
pub(crate) mod testing;
/// This is the module containing all the different tools of this app
pub(crate) mod tools;
pub(crate) mod utils;

pub use crate::models::client::ALISTRAL_CLIENT;
pub use crate::models::error::Error;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let _worker_guard = init_tracer(&cli);
    //console_subscriber::init();

    match cli.run().await {
        Ok(clean) => {
            if clean {
                ALISTRAL_CLIENT.clean_up_mb_db().await;
            }

            ExitCode::SUCCESS
        }

        Err(err) => {
            debug!("Full error with locations: {err:#?}");

            if let Some(err) = err.get_friendly_error() {
                err.print_no_panic();
            } else {
                error!("{}", snafu::Report::from_error(err));
                info!(
                    "If this reproduce multiple time, please use verbose logging before submiting your log as an issue! Without it it will be really hard to find what's wrong."
                )
            }

            ExitCode::FAILURE
        }
    }
}
