use clap::Parser;

use database::cleanup::cleanup_database;
use models::cli::Cli;
use tracing::debug;

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

pub use crate::models::client::ALISTRAL_CLIENT;
pub use crate::models::error::Error;

use crate::interface::tracing::init_tracer;
use crate::models::client::AlistralCliClient;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();
    let _worker_guard = init_tracer(&cli);
    //console_subscriber::init();

    if run_cli(cli).await {
        post_run().await
    }
}

async fn run_cli(cli: Cli) -> bool {
    debug!("Starting the database");
    // Set up the database
    let conn = &mut *ALISTRAL_CLIENT
        .musicbrainz_db
        .get_raw_connection()
        .await
        .expect("Couldn't connect to the database");

    match cli.run(conn).await {
        Result::Ok(val) => val,
        Err(err) => err.panic(),
    }
}

async fn post_run() {
    let alistral_client = AlistralCliClient::create_or_fatal().await;
    let conn = &mut *alistral_client
        .musicbrainz_db
        .get_raw_connection()
        .await
        .expect("Couldn't connect to the database");

    debug!("Cleaning some old entries...");
    cleanup_database(conn)
        .await
        .expect("Error while cleaning the database");
    debug!("Done!");
}
