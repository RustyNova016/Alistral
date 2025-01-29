use alistral_core::cli::logger::println_cli;
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
    init_tracer();
    color_eyre::install()?;

    info!("Hello world!");

    if run_cli().await {
        post_run().await
    }

    Ok(())
}

async fn run_cli() -> bool {
    debug!("Starting the database");
    // Set up the database
    let conn = &mut *ALISTRAL_CLIENT
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    let cli = Cli::parse();

    cli.run(conn).await.expect("An error occured in the app")
}

async fn post_run() {
    let alistral_client = create_client().await;
    let conn = &mut *alistral_client
        .musicbrainz_db
        .connection
        .acquire_guarded()
        .await;

    println_cli("Optional cleanup - This is fine to cancel");
    println_cli("Cleaning some old entries...");
    cleanup_database(conn)
        .await
        .expect("Error while cleaning the database");
    println_cli("Done!");

    println_cli("Have a nice day!");
}
