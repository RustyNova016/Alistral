use clap::Parser;
use color_eyre::eyre::Ok;

use database::cleanup::cleanup_database;
use models::cli::Cli;
use tracing::debug;
use tracing::info;

pub mod api;
pub mod database;
pub mod datastructures;
pub mod interface;
pub mod models;
#[cfg(test)]
pub mod testing;
/// This is the module containing all the different tools of this app
pub mod tools;
pub mod utils;

use crate::api::clients::create_client;
use crate::api::clients::ALISTRAL_CLIENT;
use crate::interface::tracing::init_tracer;
pub use crate::models::error::Error;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();
    init_tracer(&cli);
    color_eyre::install()?;

    info!("Hello world!");

    if run_cli(cli).await {
        post_run().await
    }

    Ok(())
}

async fn run_cli(cli: Cli) -> bool {
    debug!("Starting the database");
    // Set up the database
    let conn = &mut *ALISTRAL_CLIENT
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    cli.run(conn).await.expect("An error occured in the app")
}

async fn post_run() {
    let alistral_client = create_client().await;
    let conn = &mut *alistral_client
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    info!("Optional cleanup - This is fine to cancel");
    info!("Cleaning some old entries...");
    cleanup_database(conn)
        .await
        .expect("Error while cleaning the database");
    info!("Done!");

    info!("Have a nice day!");
}
